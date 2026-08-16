//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 639/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk639(t1876: f64, t708: f64, t1646: f64, t673: f64, t707: f64, t1744: f64, t4957: f64, t4971: f64, t638: f64, t5005: f64, t9: f64, t662: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7012 = t1876 * t708;
    let t7028 = t673 * t1646;
    let t7055 = t673 * t707;
    let t7181 = t4957 * t1744;
    let t7196 = t638 * t4971;
    let t7233 = t9 * t5005;
    let t7234 = t7233 * t662;
    (t7012, t7028, t7055, t7181, t7196, t7233, t7234)
}
