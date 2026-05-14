//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1296/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1296<F: Float>(t17349: F, t17351: F, t17354: F, t20705: F, t20716: F, t20719: F, t25633: F, t25636: F, t25639: F, t261: F, t237: F, t2865: F, t730: F, t7475: F, t1954: F, t723: F, t9203: F) -> (F, F, F, F) {
    let t25642 = (t17349 - 0.57685185185185185184e-1 * t17351 + 0.12361111111111111111e-1 * t17354 - 0.57685185185185185187e-1 * t20705 + 0.49444444444444444446e-1 * t20716 - 0.18541666666666666667e-1 * t20719 + 0.12361111111111111111e-1 * t25633 - 0.18541666666666666667e-1 * t25636 + 0.278125e-1 * t25639) * t261;
    let t25644 = 0.19751673498613801407e-1 * t237 * t25642;
    let t25647 = 0.23392894490538584828e1 * t730 * t2865 * t7475;
    let t25651 = 0.23392894490538584828e1 * t730 * t1954 * t9203 * t723;
    (t25642, t25644, t25647, t25651)
}
