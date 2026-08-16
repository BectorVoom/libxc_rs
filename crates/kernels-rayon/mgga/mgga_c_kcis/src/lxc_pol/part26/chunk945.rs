//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 945/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk945(t21905: f64, t4135: f64, t1468: f64, t1464: f64, t1497: f64, t6922: f64, t12322: f64, t1395: f64, t15808: f64, t1947: f64, t1394: f64, t17261: f64, t5644: f64) -> (f64, f64, f64, f64, f64) {
    let t21906 = t4135 * t21905;
    let t21907 = t1468 * t21906;
    let t21908 = t1464 * t21907;
    let t21910 = t6922 * t1497;
    let t21911 = t12322 * t21910;
    let t21912 = t1395 * t21911;
    let t21913 = t1464 * t21912;
    let t21918 = t15808 * t1947;
    let t21919 = t1394 * t21918;
    let t21922 = t17261 * t5644;
    (t21908, t21910, t21913, t21919, t21922)
}
