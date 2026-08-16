//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 1018/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk1018(t15039: f64, t79: f64, t534: f64, t1567: f64, t4509: f64, t1576: f64, t4510: f64, t13614: f64, t397: f64, t539: f64, t535: f64, t1571: f64, t4369: f64) -> (f64, f64, f64, f64, f64) {
    let t15040 = t79 * t15039;
    let t15041 = t15040 * t534;
    let t15044 = t1567 * t4509;
    let t15047 = t4510 * t1576;
    let t15050 = t397 * t13614 * t539;
    let t15052 = 0.9994882620098509563e-2_f64 * t535 * t15050;
    let t15053 = t4369 * t1571;
    (t15041, t15044, t15047, t15052, t15053)
}
