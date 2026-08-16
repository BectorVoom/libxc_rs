//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1247/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1247(t1426: f64, t6992: f64, t1421: f64, t7002: f64, t3546: f64, t7061: f64, t2601: f64, t9195: f64, t3579: f64, t6996: f64, t7059: f64, t2560: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t25654 = t1426 * t6992;
    let t25661 = t7002 * t1421;
    let t25680 = t3546 * t7061;
    let t25730 = t9195 * t2601;
    let t25737 = t3579 * t6996;
    let t25806 = t7059 * t1421;
    let t25810 = t2560 * t1421;
    (t25654, t25661, t25680, t25730, t25737, t25806, t25810)
}
