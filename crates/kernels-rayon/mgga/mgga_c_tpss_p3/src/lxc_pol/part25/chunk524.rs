//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 524/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk524(t158: f64, t2332: f64, t581: f64, t725: f64, t681: f64, t157: f64, t37: f64, t72: f64, t727: f64, t732: f64, t2211: f64, t2319: f64, t720: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2333 = t158 * t2332;
    let t2334 = t725 * t581;
    let t2335 = t681 * t2334;
    let t2337 = t37 * t157;
    let t2341 = t727 * t72;
    let t2342 = t2341 * t732;
    let t2345 = t2319 * t2211 * t720;
    (t2333, t2334, t2335, t2337, t2341, t2342, t2345)
}
