//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1187/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1187<F: Float>(t25: F, t2749: F, t33: F, t34: F, t1890: F, t9911: F, t1796: F, t1802: F, t1808: F, t1895: F, t1898: F, t1903: F, t19706: F, t19749: F, t3008: F, t3014: F, t3804: F, t3814: F, t457: F, t545: F, t572: F, t575: F, t6025: F, t6033: F, t7945: F, t9872: F, t9877: F, t9899: F, t9904: F, t9909: F) -> (F, F) {
    let t27374 = t33 * t34 / t25 / t2749;
    let t27383 = t1890 * t9911;
    let t27403 = 4.0 / 9.0 * t572 * t3014 * t9877 * t1808 + 2.0 / 27.0 * t572 * t3008 * t6033 * t3804 * t1808 - t572 * t3014 * t9899 * t1808 / 9.0 + 20.0 / 81.0 * t572 * t7945 * t19706 * t3814 * t1808 - 4.0 / 9.0 * t572 * t3008 * t9872 * t1808 - 8.0 / 81.0 * t27374 * t1895 * t1898 * t457 + 8.0 / 27.0 * t27374 * t575 * t1903 * t457 + t27383 / 81.0 + t19749 - t572 * t3008 * t9899 * t1796 / 81.0 - 5.0 / 243.0 * t572 * t7945 * t6025 * t3804 * t1808 + 2.0 / 27.0 * t572 * t3014 * t1802 * t9909 * t545 + t572 * t3014 * t9904 * t1796 / 27.0;
    (t27374, t27403)
}
