//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 991/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk991<F: Float>(t1519: F, t2839: F, t2849: F, t1128: F, t1537: F, t7984: F, t7999: F, t1112: F, t513: F, t3803: F, t1122: F, t3728: F, t2873: F, t1132: F, t1158: F, t1161: F, t2900: F, t2946: F, t2953: F, t2957: F, t3784: F, t3785: F, t3788: F, t3796: F, t7908: F, t7927: F, t8025: F, t8034: F, t8081: F, t9715: F, t9718: F, t9723: F, t9726: F, t9732: F, t9737: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t9742 = t1519 * t2839;
    let t9746 = t1519 * t2849;
    let t9747 = t1128 * t9746;
    let t9752 = t1537 * t7984;
    let t9755 = t1537 * t7999;
    let t9758 = t1112 * t513;
    let t9761 = t3803 * t513;
    let t9764 = t3728 * t1122;
    let t9767 = t1519 * t2873;
    let t9768 = t1128 * t9767;
    let t9771 = 0.176e0 * t1158 * t9715 - 0.54e0 * t8025 * t1128 * t9718 - 0.1008e1 * t7908 * t9723 - 0.176e0 * t1161 * t9726 - 0.24e0 * t2953 * t1537 * t8034 - 0.336e0 * t2957 * t9732 - 0.48e-1 * t2946 * t9732 + 800.0 / 9.0 * t3784 * t9737 + 800.0 / 9.0 * t3788 * t9737 + 0.36e0 * t7927 * t1128 * t9742 + 0.756e0 * t8081 * t9747 - 0.58666666666666666667e-1 * t1132 * t9726 - 0.48e-1 * t1158 * t9752 + 0.48e-1 * t1161 * t9755 - 100.0 / 9.0 * t9758 * t3785 - 200.0 / 9.0 * t9761 * t3785 - 0.24e-1 * t9764 * t3796 - 0.12e-1 * t2900 * t9768;
    (t9742, t9746, t9747, t9752, t9755, t9758, t9761, t9764, t9767, t9768, t9771)
}
