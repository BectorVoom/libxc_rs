//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1260/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1260(t3403: f64, t4857: f64, t15026: f64, t3623: f64, t1706: f64, t3428: f64, t135: f64, t457: f64, t4936: f64, t1174: f64, t3431: f64, t4912: f64) -> (f64, f64, f64, f64, f64) {
    let t15218 = t4857 * t3403;
    let t15245 = t15026 * t3623;
    let t15265 = t1706 * t3428;
    let t15281 = t135 * t457;
    let t15282 = t15281 * t4936;
    let t15284 = 0.55555555555555555554e-3_f64 * t1174 * t15282;
    let t15285 = t3431 * t4912;
    (t15218, t15245, t15265, t15284, t15285)
}
