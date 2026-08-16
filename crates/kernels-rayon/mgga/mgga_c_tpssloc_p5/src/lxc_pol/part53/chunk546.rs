//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 546/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk546(t491: f64, t4940: f64, t1235: f64, t1720: f64, t1721: f64, t225: f64, t1190: f64, t1751: f64, t1090: f64, t1735: f64, t3578: f64, t1216: f64, t1653: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4941 = t4940 * t491;
    let t4943 = t1720 * t1235;
    let t4945 = t1721 * t225;
    let t4947 = t1190 * t1751;
    let t4949 = t1735 * t1090;
    let t4950 = t3578 * t4949;
    let t4953 = t1653 * t1216;
    (t4941, t4943, t4945, t4947, t4950, t4953)
}
