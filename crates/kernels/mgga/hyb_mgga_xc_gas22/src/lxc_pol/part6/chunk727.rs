//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 727/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk727<F: Float>(t3827: F, t54: F, t3844: F, t588: F, t57: F, t592: F, t60: F, t596: F, t63: F, t600: F, t66: F, t604: F, t69: F, t608: F, t1941: F, t612: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3847 = t54 * t3827;
    let t3849 = t588 * t3844;
    let t3851 = t57 * t3827;
    let t3853 = t592 * t3844;
    let t3855 = t60 * t3827;
    let t3857 = t596 * t3844;
    let t3859 = t63 * t3827;
    let t3861 = t600 * t3844;
    let t3863 = t66 * t3827;
    let t3865 = t604 * t3844;
    let t3867 = t69 * t3827;
    let t3869 = t608 * t3844;
    let t3871 = t1941 * t3827;
    let t3873 = t612 * t3844;
    (t3847, t3849, t3851, t3853, t3855, t3857, t3859, t3861, t3863, t3865, t3867, t3869, t3871, t3873)
}
