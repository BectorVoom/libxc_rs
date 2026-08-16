//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 976/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk976(t4468: f64, t6207: f64, t15934: f64, t12861: f64, t2109: f64, t4312: f64, t11838: f64, t15913: f64, t15916: f64, t1592: f64, t15921: f64, t15927: f64, t15932: f64, t15939: f64, t15941: f64, t15944: f64, t15947: f64, t15950: f64, t15953: f64, t15958: f64, t15961: f64, t15964: f64, t15968: f64, t4414: f64) -> (f64, f64, f64, f64) {
    let t17969 = t6207 * t4468;
    let t17973 = 0.15476481481481481481e-2_f64 * t15934;
    let t17980 = t2109 * t12861;
    let t17981 = t17980 * t4312;
    let t17988 = -0.15476481481481481481e-2_f64 * t11838 + 0.77382407407407407406e-3_f64 * t15913 - 0.34822083333333333332e-2_f64 * t15916 + 0.23214722222222222222e-2_f64 * t15921 - 0.15476481481481481481e-2_f64 * t15927 + 0.66725e-1_f64 * t1592 * t17969 + 0.51588271604938271604e-3_f64 * t15932 - t17973 + 0.34822083333333333332e-2_f64 * t15939 + 0.46429444444444444443e-2_f64 * t15941 + 0.11607361111111111111e-2_f64 * t15944 + 0.19345601851851851852e-2_f64 * t15947 + 0.11607361111111111111e-2_f64 * t15950 - 0.46429444444444444444e-2_f64 * t15953 - 0.2671335375e-1_f64 * t4414 * t17981 + 0.46429444444444444443e-2_f64 * t15958 - 0.23214722222222222222e-2_f64 * t15961 - 0.77382407407407407406e-3_f64 * t15964 + 0.38691203703703703704e-2_f64 * t15968;
    (t17969, t17980, t17981, t17988)
}
