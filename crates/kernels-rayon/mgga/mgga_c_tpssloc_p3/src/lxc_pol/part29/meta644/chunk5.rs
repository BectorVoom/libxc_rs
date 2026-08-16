//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2127/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2127(t13191: f64, t25119: f64, t841: f64, t1878: f64, t81982: f64, t13184: f64, t221: f64, t25120: f64, t6604: f64, t81962: f64, t13196: f64, t13204: f64, t6581: f64) -> (f64, f64, f64, f64, f64) {
    let t87418 = t25119 * t841 * t13191;
    let t87420 = t1878 * t81982;
    let t87422 = t87420 * t221 * t13184;
    let t87425 = t81962 * t6604 * t25120;
    let t87426 = 0.11869590291677274911e0_f64 * t87425;
    let t87428 = t25119 * t841 * t13196;
    let t87430 = t6581 * t13204;
    (t87418, t87422, t87426, t87428, t87430)
}
