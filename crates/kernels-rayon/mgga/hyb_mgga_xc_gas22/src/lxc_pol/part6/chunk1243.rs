//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1243/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1243(t1056: f64, t3466: f64, t2520: f64, t3474: f64, t1396: f64, t7147: f64, t2477: f64, t7074: f64, t9266: f64, t948: f64, t260: f64, t9031: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t25245 = 8.0_f64 * t3466 * t1056;
    let t25257 = t3474 * t2520;
    let t25262 = t1396 * t7147;
    let t25267 = t3474 * t2477;
    let t25270 = t1396 * t7074;
    let t25273 = t9266 * t948;
    let t25276 = t260 * t9031;
    (t25245, t25257, t25262, t25267, t25270, t25273, t25276)
}
