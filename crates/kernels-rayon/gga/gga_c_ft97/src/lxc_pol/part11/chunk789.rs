//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 789/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk789(t2409: f64, t835: f64, t882: f64, t1882: f64, t2854: f64, t2749: f64, t2801: f64, t296: f64, t192: f64, t7640: f64, t10262: f64, t319: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10675 = t835 * t882 * t2409;
    let t10678 = t1882 * t2854;
    let t10680 = t2749 * t2801;
    let t10681 = t296 * t10680;
    let t10683 = t192 * t7640;
    let t10685 = t10683 * t319 * t10262;
    (t10675, t10678, t10680, t10681, t10683, t10685)
}
