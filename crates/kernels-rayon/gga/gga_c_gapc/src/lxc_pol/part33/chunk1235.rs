//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 1235/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk1235(t11418: f64, t128: f64, t567: f64, t5741: f64, t681: f64, t35269: f64, t35272: f64, t35275: f64, t35277: f64, t35280: f64, t35283: f64, t35285: f64, t35287: f64, t35289: f64, t35293: f64) -> f64 {
    let t35298 = t11418 * t5741 * t681 * t128 * t567;
    let t35300 = 0.42206481990611010728e-7_f64 * t35269 + 0.40022999988963401106e-7_f64 * t35272 + 0.40096157891080460192e-6_f64 * t35275 - 0.10258519928273509552e-8_f64 * t35277 - 0.16908181191593721013e-5_f64 * t35280 + 0.80192315782160920384e-6_f64 * t35283 + 0.63309722985916516092e-7_f64 * t35285 - 0.19336854506021130164e-7_f64 * t35287 - 0.27041506680806477869e-6_f64 * t35289 - 0.94685814672924837675e-4_f64 * t35293 + 0.94685814672924837675e-4_f64 * t35298;
    t35300
}
