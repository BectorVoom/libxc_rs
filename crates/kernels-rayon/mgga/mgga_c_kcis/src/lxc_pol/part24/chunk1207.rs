//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1207/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1207(t96123: f64, t96137: f64, t1250: f64, t251: f64, t47323: f64, t96217: f64, t15216: f64, t28101: f64, t26960: f64, t1268: f64, t9494: f64, t26955: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t97273 = 0.23214722222222222222e-2_f64 * t96123;
    let t97281 = 0.23214722222222222222e-2_f64 * t96137;
    let t97297 = t47323 * t251 * t1250;
    let t97312 = 0.15476481481481481481e-2_f64 * t96217;
    let t97330 = t15216 * t28101;
    let t97332 = 0.7722800925925925926e-4_f64 * t26960 * t97330;
    let t97338 = t1268 * t9494;
    let t97344 = 0.10306077835648148148e-4_f64 * t26955 * t97330;
    (t97273, t97281, t97297, t97312, t97332, t97338, t97344)
}
