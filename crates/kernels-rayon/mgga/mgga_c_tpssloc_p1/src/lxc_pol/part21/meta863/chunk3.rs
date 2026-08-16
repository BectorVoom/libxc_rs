//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3144/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3144(t11583: f64, t17635: f64, t11570: f64, t17691: f64, t15372: f64, t4889: f64, t11529: f64, t1174: f64, t6126: f64, t11569: f64, t15278: f64, t15288: f64, t15357: f64, t15360: f64, t18416: f64, t3447: f64, t3449: f64, t3469: f64, t3475: f64, t460: f64, t4919: f64, t4934: f64, t52216: f64, t52220: f64, t6144: f64, t8034: f64) -> f64 {
    let t65077 = t11583 * t17635;
    let t65087 = t11570 * t17691;
    let t65093 = t4889 * t15372;
    let t65112 = t1174 * t11529 * t6126;
    let t65114 = 0.55555555555555555554e-3_f64 * t3447 * t18416 * t15288 + 0.11111111111111111111e-2_f64 * t3447 * t3449 * t65077 + 0.22222222222222222222e-2_f64 * t3447 * t4919 * t52216 + 0.11111111111111111111e-2_f64 * t3447 * t4919 * t52220 - 0.14814814814814814814e-2_f64 * t3447 * t11569 * t65087 + 0.44444444444444444444e-2_f64 * t4889 * t15360 + 0.2962962962962962963e-2_f64 * t65093 - 0.16666666666666666666e-2_f64 * t1174 * t4934 * t8034 * t15357 - 0.83333333333333333332e-3_f64 * t1174 * t4934 * t6144 * t3469 * t460 - 0.83333333333333333332e-3_f64 * t1174 * t4934 * t6144 * t3475 * t460 + 0.14814814814814814814e-2_f64 * t4889 * t15278 + 0.12345679012345679012e-3_f64 * t65112;
    t65114
}
