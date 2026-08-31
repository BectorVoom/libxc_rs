//! C libxc oracle parity tests for libxc-reval vs libxc-sys.

use libxc_core::input::{GgaInput, LdaInput};
use libxc_core::model::{DerivativeOrder, Spin, Thresholds};
use libxc_core::output::{GgaOutput, LdaOutput};
use libxc_core::registry::lookup_by_name;
use libxc_sys::*;

const LDA_CORPUS: &[&str] = &["lda_x", "lda_c_vwn", "lda_c_pw", "lda_c_pz", "lda_c_hl"];

const GGA_CORPUS: &[&str] = &[
    "gga_x_pbe",
    "gga_c_pbe",
    "gga_x_b88",
    "gga_c_lyp",
    "gga_x_rpbe",
    "gga_k_lgap",
    "gga_k_dk",
];

#[test]
fn test_oracle_parity_lda() {
    let rho = [0.001, 0.01, 0.05, 0.1, 0.2, 0.5, 1.0, 2.0, 5.0, 10.0];
    let np = rho.len();
    let thresholds = Thresholds::default();

    for &name in LDA_CORPUS {
        let id = lookup_by_name(name).unwrap_or_else(|_| panic!("lookup {name}"));
        let mut zk_rust = vec![0.0; np];
        let mut vrho_rust = vec![0.0; np];
        let input = LdaInput::new(&rho, np, Spin::Unpolarized).unwrap();
        let mut output = LdaOutput::new(
            Some(&mut zk_rust),
            Some(&mut vrho_rust),
            None,
            None,
            None,
            np,
            Spin::Unpolarized,
        )
        .unwrap();

        libxc_reval::routing::dispatch_lda_by_name(
            name,
            &input,
            &mut output,
            DerivativeOrder::Vxc,
            Spin::Unpolarized,
            &thresholds,
        )
        .unwrap_or_else(|| panic!("failed to dispatch {name}"))
        .unwrap_or_else(|e| panic!("error in {name}: {e}"));

        // Oracle C libxc
        unsafe {
            let mut c_func = std::mem::MaybeUninit::uninit();
            let ret = xc_func_init(c_func.as_mut_ptr(), id.raw() as i32, XC_UNPOLARIZED as i32);
            assert_eq!(ret, 0, "C libxc init {name} (id {})", id.raw());
            let mut c_func = c_func.assume_init();

            let mut zk_c = vec![0.0; np];
            let mut vrho_c = vec![0.0; np];
            xc_lda_exc_vxc(
                &c_func,
                np,
                rho.as_ptr(),
                zk_c.as_mut_ptr(),
                vrho_c.as_mut_ptr(),
            );
            xc_func_end(&mut c_func);

            for i in 0..np {
                let err_zk = (zk_rust[i] - zk_c[i]).abs();
                let rel_zk = if zk_c[i].abs() > 1e-14 {
                    err_zk / zk_c[i].abs()
                } else {
                    err_zk
                };
                assert!(
                    rel_zk < 1e-13 || err_zk < 1e-14,
                    "{name} zk[{i}] mismatch: rust={}, c={}, diff={err_zk}, rel={rel_zk}",
                    zk_rust[i],
                    zk_c[i]
                );

                let err_vrho = (vrho_rust[i] - vrho_c[i]).abs();
                let rel_vrho = if vrho_c[i].abs() > 1e-14 {
                    err_vrho / vrho_c[i].abs()
                } else {
                    err_vrho
                };
                assert!(
                    rel_vrho < 1e-13 || err_vrho < 1e-14,
                    "{name} vrho[{i}] mismatch: rust={}, c={}, diff={err_vrho}, rel={rel_vrho}",
                    vrho_rust[i],
                    vrho_c[i]
                );
            }
        }
    }
}

#[test]
fn test_oracle_parity_gga() {
    let rho = [0.01, 0.05, 0.1, 0.2, 0.5, 1.0];
    let sigma = [0.001, 0.005, 0.01, 0.04, 0.1, 0.5];
    let np = rho.len();
    let thresholds = Thresholds::default();

    for &name in GGA_CORPUS {
        let id = lookup_by_name(name).unwrap_or_else(|_| panic!("lookup {name}"));
        let mut zk_rust = vec![0.0; np];
        let mut vrho_rust = vec![0.0; np];
        let mut vsigma_rust = vec![0.0; np];
        let input = GgaInput::new(&rho, &sigma, np, Spin::Unpolarized).unwrap();
        let mut output = GgaOutput::new(
            Some(&mut zk_rust),
            Some(&mut vrho_rust),
            Some(&mut vsigma_rust),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            np,
            Spin::Unpolarized,
        )
        .unwrap();

        libxc_reval::routing::dispatch_gga_by_name(
            name,
            &input,
            &mut output,
            DerivativeOrder::Vxc,
            Spin::Unpolarized,
            &thresholds,
        )
        .unwrap_or_else(|| panic!("failed to dispatch {name}"))
        .unwrap_or_else(|e| panic!("error in {name}: {e}"));

        // Oracle C libxc
        unsafe {
            let mut c_func = std::mem::MaybeUninit::uninit();
            let ret = xc_func_init(c_func.as_mut_ptr(), id.raw() as i32, XC_UNPOLARIZED as i32);
            assert_eq!(ret, 0, "C libxc init {name} (id {})", id.raw());
            let mut c_func = c_func.assume_init();

            let mut zk_c = vec![0.0; np];
            let mut vrho_c = vec![0.0; np];
            let mut vsigma_c = vec![0.0; np];
            xc_gga_exc_vxc(
                &c_func,
                np,
                rho.as_ptr(),
                sigma.as_ptr(),
                zk_c.as_mut_ptr(),
                vrho_c.as_mut_ptr(),
                vsigma_c.as_mut_ptr(),
            );
            xc_func_end(&mut c_func);

            for i in 0..np {
                let err_zk = (zk_rust[i] - zk_c[i]).abs();
                let rel_zk = if zk_c[i].abs() > 1e-14 {
                    err_zk / zk_c[i].abs()
                } else {
                    err_zk
                };
                assert!(
                    rel_zk < 1e-13 || err_zk < 1e-14,
                    "{name} zk[{i}] mismatch: rust={}, c={}, diff={err_zk}, rel={rel_zk}",
                    zk_rust[i],
                    zk_c[i]
                );

                let err_vrho = (vrho_rust[i] - vrho_c[i]).abs();
                let rel_vrho = if vrho_c[i].abs() > 1e-14 {
                    err_vrho / vrho_c[i].abs()
                } else {
                    err_vrho
                };
                assert!(
                    rel_vrho < 1e-13 || err_vrho < 1e-14,
                    "{name} vrho[{i}] mismatch: rust={}, c={}, diff={err_vrho}, rel={rel_vrho}",
                    vrho_rust[i],
                    vrho_c[i]
                );

                let err_vsigma = (vsigma_rust[i] - vsigma_c[i]).abs();
                let rel_vsigma = if vsigma_c[i].abs() > 1e-14 {
                    err_vsigma / vsigma_c[i].abs()
                } else {
                    err_vsigma
                };
                assert!(
                    rel_vsigma < 1e-13 || err_vsigma < 1e-14,
                    "{name} vsigma[{i}] mismatch: rust={}, c={}, diff={err_vsigma}, rel={rel_vsigma}",
                    vsigma_rust[i],
                    vsigma_c[i]
                );
            }
        }
    }
}
