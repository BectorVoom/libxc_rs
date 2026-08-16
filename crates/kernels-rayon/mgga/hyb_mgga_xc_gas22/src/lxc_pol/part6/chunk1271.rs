//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1271/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1271(t25: f64, t2749: f64, t33: f64, t34: f64, t1890: f64, t9911: f64, t1796: f64, t1802: f64, t1808: f64, t1895: f64, t1898: f64, t1903: f64, t19706: f64, t19749: f64, t3008: f64, t3014: f64, t3804: f64, t3814: f64, t457: f64, t545: f64, t572: f64, t575: f64, t6025: f64, t6033: f64, t7945: f64, t9872: f64, t9877: f64, t9899: f64, t9904: f64, t9909: f64) -> (f64, f64) {
    let t27374 = t33 * t34 / t25 / t2749;
    let t27383 = t1890 * t9911;
    let t27403 = 4.0_f64 / 9.0_f64 * t572 * t3014 * t9877 * t1808 + 2.0_f64 / 27.0_f64 * t572 * t3008 * t6033 * t3804 * t1808 - t572 * t3014 * t9899 * t1808 / 9.0_f64 + 20.0_f64 / 81.0_f64 * t572 * t7945 * t19706 * t3814 * t1808 - 4.0_f64 / 9.0_f64 * t572 * t3008 * t9872 * t1808 - 8.0_f64 / 81.0_f64 * t27374 * t1895 * t1898 * t457 + 8.0_f64 / 27.0_f64 * t27374 * t575 * t1903 * t457 + t27383 / 81.0_f64 + t19749 - t572 * t3008 * t9899 * t1796 / 81.0_f64 - 5.0_f64 / 243.0_f64 * t572 * t7945 * t6025 * t3804 * t1808 + 2.0_f64 / 27.0_f64 * t572 * t3014 * t1802 * t9909 * t545 + t572 * t3014 * t9904 * t1796 / 27.0_f64;
    (t27374, t27403)
}
