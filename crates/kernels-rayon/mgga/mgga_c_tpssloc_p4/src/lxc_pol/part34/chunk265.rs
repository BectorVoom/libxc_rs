//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 265/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk265(t419: f64, t409: f64, t410: f64, t1086: f64, t407: f64, t281: f64, t415: f64, t904: f64, t241: f64, t457: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1097 = t419 * t419;
    let t1098 = 1.0_f64 / t1097;
    let t1099 = t409 * t1098;
    let t1100 = 1.0_f64 / t410;
    let t1105 = 0.29896666666666666667e0_f64 * t1086;
    let t1107 = f64::sqrt(t407);
    let t1111 = t281 * t904 * t415;
    let t1112 = 0.82156666666666666667e-1_f64 * t1111;
    let t1113 = t241 * t457;
    (t1097, t1098, t1099, t1100, t1105, t1107, t1111, t1112, t1113)
}
