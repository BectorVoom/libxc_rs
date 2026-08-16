//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3503/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3503(t1043: f64, t5819: f64, t54397: f64, t1045: f64, t4186: f64, t53585: f64, t1063: f64, t1066: f64, t11859: f64, t15696: f64, t15700: f64, t15974: f64, t16048: f64, t16222: f64, t16509: f64, t19501: f64, t247: f64, t3117: f64, t42328: f64, t42450: f64, t42481: f64, t43116: f64, t4896: f64, t53710: f64, t53724: f64, t53762: f64, t53771: f64, t53790: f64, t54658: f64, t63330: f64) -> (f64, f64, f64, f64) {
    let t66061 = t5819 * t1043;
    let t66062 = t66061 * t54397;
    let t66066 = t1045 * t4186;
    let t66067 = t53585 * t66066;
    let t66086 = -0.91464571985215438873e-2_f64 * t16509 * t16048 * t4896 - 0.3811023832717309953e-3_f64 * t53710 + 0.67751534803863288054e-3_f64 * t53724 - 0.52930886565518193792e-4_f64 * t42450 - 0.28582678745379824648e-2_f64 * t15700 * t54658 * t66062 + 0.95275595817932748828e-3_f64 * t15700 * t16222 * t66067 + 0.28582678745379824648e-3_f64 * t42328 * t15696 * t15974 + 0.84689418504829110066e-4_f64 * t53762 - 0.28582678745379824648e-3_f64 * t53771 + 0.47637797908966374413e-4_f64 * t42481 - 0.3811023832717309953e-3_f64 * t53790 - 0.57165357490759649296e-3_f64 * t1063 * t247 * t1066 * t63330 - 0.42874018118069736972e-3_f64 * t11859 * t3117 * t19501 * t43116;
    (t66061, t66062, t66067, t66086)
}
