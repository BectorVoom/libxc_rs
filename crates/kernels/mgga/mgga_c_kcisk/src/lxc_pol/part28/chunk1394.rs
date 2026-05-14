//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1394/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1394<F: Float>(t116123: F, t116416: F, t122054: F, t122060: F, t122063: F, t122065: F, t122068: F, t122071: F, t122074: F, t122076: F, t1693: F, t20: F, t23748: F, t23834: F, t23922: F, t2785: F, t34125: F, t34261: F, t35242: F, t4830: F, t654: F, t7261: F, t9656: F, t9664: F, t9670: F, t9940: F) -> (F,) {
    let t122096 = 0.88437037037037037033e-2 * t122054 - 0.10416666666666666667e-1 * t23922 * t9656 * t2785 - 0.24320185185185185185e-1 * t122060 + 0.1621345679012345679e-1 * t122063 + 0.33163888888888888888e-2 * t122065 + 0.13265555555555555555e-1 * t122068 - 0.88437037037037037033e-2 * t122071 + 0.33163888888888888888e-2 * t122074 - 0.69444444444444444447e-2 * t122076 - 0.10416666666666666667e-1 * t4830 * t35242 * t2785 - 0.10416666666666666667e-1 * t1693 * t23748 * t654 * t20 * t2785 + 0.10416666666666666667e-1 * t9664 * t7261 * t9670 * t23834 - 0.55555555555555555558e-1 * t116416 * t9940 - 0.55555555555555555558e-1 * t116123 * t9940 - 0.55555555555555555558e-1 * t34125 * t34261;
    (t122096,)
}
