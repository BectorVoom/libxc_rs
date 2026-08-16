//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1270/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1270(t3840: f64, t6012: f64, t3836: f64, t3832: f64, t1179: f64, t1796: f64, t1808: f64, t1895: f64, t1897: f64, t1898: f64, t1903: f64, t19756: f64, t23295: f64, t23341: f64, t23351: f64, t3008: f64, t3014: f64, t3023: f64, t3814: f64, t545: f64, t572: f64, t575: f64, t7945: f64, t9872: f64, t9877: f64, t9888: f64, t9909: f64) -> f64 {
    let t27333 = t6012 * t3840;
    let t27335 = t6012 * t3836;
    let t27341 = t6012 * t3832;
    let t27348 = -t572 * t3014 * t9888 * t1796 / 9.0_f64 - 5.0_f64 / 243.0_f64 * t572 * t7945 * t9872 * t1796 - 40.0_f64 / 729.0_f64 * t572 * t23295 * t19756 * t3814 * t1808 + 2.0_f64 / 27.0_f64 * t572 * t3008 * t9877 * t1796 - 142.0_f64 / 243.0_f64 * t23341 + 28.0_f64 / 729.0_f64 * t23351 + 4.0_f64 / 27.0_f64 * t3023 * t575 * t1903 * t1179 - 2.0_f64 / 243.0_f64 * t27333 + 4.0_f64 / 243.0_f64 * t27335 - 4.0_f64 / 81.0_f64 * t3023 * t1895 * t1898 * t1179 - 4.0_f64 / 729.0_f64 * t27341 - 2.0_f64 / 81.0_f64 * t572 * t3008 * t1897 * t9909 * t545;
    t27348
}
