//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1114/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1114(t11802: f64, t37685: f64, t39352: f64, t39355: f64, t39358: f64, t39362: f64, t39364: f64, t39367: f64, t39370: f64, t39373: f64, t39376: f64, t39379: f64) -> f64 {
    let t39381 = t37685 * t11802;
    let t39383 = -0.16463622957338778997e0_f64 * t39352 - 0.14282990759302185291e-1_f64 * t39355 - 0.57131963037208741166e-1_f64 * t39358 - t39362 + 0.43341108700271342816e-1_f64 * t39364 + 0.13002332610081402845e0_f64 * t39367 - 0.86682217400542685632e-1_f64 * t39370 - 0.86682217400542685632e-1_f64 * t39373 + 0.86682217400542685632e-1_f64 * t39376 + 0.2600466522016280569e0_f64 * t39379 + 0.86682217400542685632e-1_f64 * t39381;
    t39383
}
