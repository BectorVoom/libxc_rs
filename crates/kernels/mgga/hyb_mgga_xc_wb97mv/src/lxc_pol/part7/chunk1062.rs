//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1062/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1062<F: Float>(t11035: F, t787: F, t10917: F, t10921: F, t10923: F, t10926: F, t10929: F, t10932: F, t10935: F, t10939: F, t10942: F, t10946: F, t10949: F, t10953: F, t10955: F, t10957: F, t10959: F, t10981: F, t260: F, t855: F) -> (F, F) {
    let t11037 = 1.0 * t787 * t11035;
    let t11038 = 0.10389515463408878255e3 * t855 * t10917 - t10921 + t10923 + t10926 - t10929 - t10932 - t10935 + t10939 + t10942 + t10946 - 0.10254018858216406658e4 * t855 * t10949 + t10953 + t10955 - t10957 + t10959 + 0.19751673498613801407e-1 * t260 * t10981 + t11037;
    (t11037, t11038)
}
