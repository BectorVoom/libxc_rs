//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1347/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1347(t2967: f64, t31767: f64, t2822: f64, t3832: f64, t7063: f64, t10529: f64, t8613: f64, t24915: f64, t3568: f64, t12062: f64, t4908: f64, t12277: f64, t2469: f64, t36119: f64, t36122: f64, t36266: f64, t36269: f64, t36270: f64, t36271: f64, t36272: f64, t36275: f64, t36280: f64, t36283: f64, t36285: f64, t36288: f64, t972: f64) -> (f64, f64, f64, f64) {
    let t36290 = 4.0_f64 * t31767 * t2967;
    let t36293 = 6.0_f64 * t7063 * t3832 * t2822;
    let t36295 = 4.0_f64 * t10529 * t8613;
    let t36297 = 4.0_f64 * t24915 * t3568;
    let t36299 = 4.0_f64 * t4908 * t12062;
    let t36300 = 4.0_f64 * t12277 * t2469 * t972 - t36119 - t36122 + t36266 - t36269 + t36270 + t36271 - t36272 + t36275 + t36280 - t36283 + t36285 - t36288 - t36290 - t36293 - t36295 + t36297 - t36299;
    (t36290, t36295, t36299, t36300)
}
