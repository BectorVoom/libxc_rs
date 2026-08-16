//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1015/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1015(t1070: f64, t2938: f64, t1276: f64, t2983: f64, t352: f64, t12428: f64, t3275: f64, t3472: f64, t12086: f64, t3579: f64, t12570: f64, t3262: f64, t3465: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12598 = t1070 * t2938;
    let t12599 = t1276 * t12598;
    let t12683 = t352 * t2983;
    let t12720 = t3275 * t3472 * t12428;
    let t12721 = 5.0_f64 / 16.0_f64 * t12720;
    let t12722 = t3579 * t12086;
    let t12723 = t12722 / 2.0_f64;
    let t12725 = t3262 * t3465 * t12570;
    (t12598, t12599, t12683, t12721, t12723, t12725)
}
