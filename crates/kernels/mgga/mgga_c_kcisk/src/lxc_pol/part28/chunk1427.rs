//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1427/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1427<F: Float>(t35394: F, t4998: F, t9740: F, t117671: F, t2028: F, t7261: F, t9162: F, t123: F, t2801: F, t35408: F, t117705: F, t121454: F, t121460: F, t121463: F, t121468: F, t121489: F, t121492: F, t121495: F, t122638: F, t2804: F, t2807: F, t34473: F, t35431: F, t35463: F, t9728: F, t9743: F, t9748: F, t9999: F) -> (F, F) {
    let t122814 = t9740 * t4998 * t35394;
    let t122818 = t7261 * t117671 * t9162 * t2028;
    let t122822 = t2801 * t35408 * t123;
    let t122830 = 0.52083333333333333333e-2 * t2804 * t122638 + 0.52083333333333333333e-2 * t35463 * t9728 + 0.52083333333333333333e-2 * t35431 * t9748 - 0.11607361111111111111e-2 * t121454 + 0.11607361111111111111e-2 * t121460 - 0.38691203703703703703e-3 * t121463 - 0.10416666666666666667e-1 * t34473 * t9999 * t2807 - 0.15476481481481481481e-2 * t121468 - 0.5787037037037037037e-3 * t122814 + 0.898632125e-3 * t117705 * t122818 + 0.92592592592592592593e-2 * t122822 * t9743 + 0.31250000000000000001e-1 * t9740 * t122818 - 0.61905925925925925925e-2 * t121489 + 0.12381185185185185185e-1 * t121492 - 0.23214722222222222222e-2 * t121495;
    (t122818, t122830)
}
