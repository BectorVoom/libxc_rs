//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 699/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk699(t1736: f64, t4953: f64, t1744: f64, t4929: f64, t4956: f64, t633: f64, t630: f64, t4957: f64, t45: f64, t4920: f64, t1704: f64, t4907: f64) -> (f64, f64, f64, f64, f64) {
    let t10902 = 1.0_f64 / t4953 / t1736;
    let t10903 = t4929 * t1744;
    let t10906 = 1.0_f64 / t4956 / t633;
    let t10907 = t10902 * t10903 * t10906;
    let t10913 = 1.0_f64 / t4953 / t630;
    let t10915 = t10913 * t10903 * t4957;
    let t10918 = t45 * t4920;
    let t10924 = 1.0_f64 / t4907 / t1704;
    (t10903, t10907, t10915, t10918, t10924)
}
