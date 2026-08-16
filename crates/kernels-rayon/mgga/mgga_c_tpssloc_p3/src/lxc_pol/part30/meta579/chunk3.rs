//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1958/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1958(t28896: f64, t3941: f64, t1873: f64, t5493: f64, t1401: f64, t28017: f64, t1458: f64, t23880: f64, t26523: f64, t28868: f64, t28888: f64, t28890: f64, t28892: f64, t28895: f64, t5456: f64, t577: f64, t7010: f64) -> (f64, f64) {
    let t28898 = 54.0_f64 * t3941 * t28896;
    let t28899 = t1873 * t5493;
    let t28901 = 27.0_f64 * t3941 * t28899;
    let t28903 = 0.135e2_f64 * t1401 * t28017;
    let t28904 = 0.45e1_f64 * t28868 * t577 + 27.0_f64 * t26523 * t1458 + 27.0_f64 * t23880 * t5456 + 0.135e2_f64 * t7010 * t5493 + t28888 + t28890 + t28892 + t28895 + t28898 + t28901 + t28903;
    (t28899, t28904)
}
