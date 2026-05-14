//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 888/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk888<F: Float>(t4468: F, t6207: F, t15934: F, t12861: F, t2109: F, t4312: F, t11838: F, t15913: F, t15916: F, t1592: F, t15921: F, t15927: F, t15932: F, t15939: F, t15941: F, t15944: F, t15947: F, t15950: F, t15953: F, t15958: F, t15961: F, t15964: F, t15968: F, t4414: F) -> (F, F, F, F) {
    let t17969 = t6207 * t4468;
    let t17973 = 0.15476481481481481481e-2 * t15934;
    let t17980 = t2109 * t12861;
    let t17981 = t17980 * t4312;
    let t17988 = -0.15476481481481481481e-2 * t11838 + 0.77382407407407407406e-3 * t15913 - 0.34822083333333333332e-2 * t15916 + 0.23214722222222222222e-2 * t15921 - 0.15476481481481481481e-2 * t15927 + 0.66725e-1 * t1592 * t17969 + 0.51588271604938271604e-3 * t15932 - t17973 + 0.34822083333333333332e-2 * t15939 + 0.46429444444444444443e-2 * t15941 + 0.11607361111111111111e-2 * t15944 + 0.19345601851851851852e-2 * t15947 + 0.11607361111111111111e-2 * t15950 - 0.46429444444444444444e-2 * t15953 - 0.2671335375e-1 * t4414 * t17981 + 0.46429444444444444443e-2 * t15958 - 0.23214722222222222222e-2 * t15961 - 0.77382407407407407406e-3 * t15964 + 0.38691203703703703704e-2 * t15968;
    (t17969, t17980, t17981, t17988)
}
