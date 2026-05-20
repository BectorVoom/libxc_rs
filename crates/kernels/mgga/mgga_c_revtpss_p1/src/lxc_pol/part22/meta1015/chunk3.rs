//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3503/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3503<F: Float>(t1043: F, t5819: F, t54397: F, t1045: F, t4186: F, t53585: F, t1063: F, t1066: F, t11859: F, t15696: F, t15700: F, t15974: F, t16048: F, t16222: F, t16509: F, t19501: F, t247: F, t3117: F, t42328: F, t42450: F, t42481: F, t43116: F, t4896: F, t53710: F, t53724: F, t53762: F, t53771: F, t53790: F, t54658: F, t63330: F) -> (F, F, F, F) {
    let t66061 = t5819 * t1043;
    let t66062 = t66061 * t54397;
    let t66066 = t1045 * t4186;
    let t66067 = t53585 * t66066;
    let t66086 = -F::cast_from(0.91464571985215438873e-2_f64) * t16509 * t16048 * t4896 - F::cast_from(0.3811023832717309953e-3_f64) * t53710 + F::cast_from(0.67751534803863288054e-3_f64) * t53724 - F::cast_from(0.52930886565518193792e-4_f64) * t42450 - F::cast_from(0.28582678745379824648e-2_f64) * t15700 * t54658 * t66062 + F::cast_from(0.95275595817932748828e-3_f64) * t15700 * t16222 * t66067 + F::cast_from(0.28582678745379824648e-3_f64) * t42328 * t15696 * t15974 + F::cast_from(0.84689418504829110066e-4_f64) * t53762 - F::cast_from(0.28582678745379824648e-3_f64) * t53771 + F::cast_from(0.47637797908966374413e-4_f64) * t42481 - F::cast_from(0.3811023832717309953e-3_f64) * t53790 - F::cast_from(0.57165357490759649296e-3_f64) * t1063 * t247 * t1066 * t63330 - F::cast_from(0.42874018118069736972e-3_f64) * t11859 * t3117 * t19501 * t43116;
    (t66061, t66062, t66067, t66086)
}
