//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1114/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1114(t40260: f64, t3578: f64, t494: f64, t97: f64, t113: f64, t11505: f64, t11588: f64, t38355: f64, t11592: f64, t37400: f64, t10680: f64, t11587: f64, t37421: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40261 = 0.13972381860938637374e0_f64 * t40260;
    let t40276 = t97 * t3578 * t494;
    let t40282 = t97 * t11505 * t113;
    let t40303 = t38355 * t11588;
    let t40305 = t37400 * t11592;
    let t40308 = t10680 * t11587 * t37421;
    (t40261, t40276, t40282, t40303, t40305, t40308)
}
