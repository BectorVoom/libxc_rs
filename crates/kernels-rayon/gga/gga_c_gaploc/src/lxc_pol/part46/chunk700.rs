//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 700/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk700(t12528: f64, t12542: f64, t10608: f64, t3177: f64, t9272: f64, t12533: f64, t12536: f64, t12539: f64, t12935: f64, t12936: f64, t12937: f64, t12941: f64, t12944: f64, t12946: f64) -> (f64, f64) {
    let t12948 = 0.11502877786176224903e1_f64 * t12528;
    let t12952 = 0.19171462976960374838e1_f64 * t12542;
    let t12953 = t10608 * t3177;
    let t12954 = t9272 * t12953;
    let t12955 = 0.11502877786176224903e1_f64 * t12954;
    let t12956 = -t12935 + t12936 + t12937 - t12941 - 0.29792074959875355558e-1_f64 * t12944 + 0.29792074959875355558e-1_f64 * t12946 - t12948 + 0.38342925953920749676e0_f64 * t12533 - 0.38342925953920749676e0_f64 * t12536 - 0.76685851907841499352e0_f64 * t12539 + t12952 - t12955;
    (t12953, t12956)
}
