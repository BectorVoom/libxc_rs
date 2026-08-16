//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1237/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1237(t19705: f64, t4873: f64, t3092: f64, t357: f64, t4866: f64, t4893: f64, t3117: f64, t19450: f64, t4900: f64, t11661: f64, t19501: f64, t1047: f64, t1063: f64, t12013: f64, t16067: f64, t16089: f64, t19688: f64, t19693: f64, t19697: f64, t19702: f64, t3127: f64, t4803: f64, t4808: f64, t4834: f64, t4892: f64, t4899: f64, t6308: f64) -> f64 {
    let t19706 = t19705 * t4873;
    let t19707 = t3092 * t19706;
    let t19716 = t357 * t4866;
    let t19717 = t4893 * t19716;
    let t19718 = t3117 * t19717;
    let t19721 = t19450 * t4900;
    let t19722 = t3117 * t19721;
    let t19725 = t19501 * t11661;
    let t19726 = t3092 * t19725;
    let t19729 = 0.23818898954483187207e-3_f64 * t1063 * t19688 - 0.23818898954483187207e-3_f64 * t3127 * t19693 + 0.21437009059034868486e-3_f64 * t19697 * t1047 - 0.14291339372689912324e-3_f64 * t3127 * t19702 + 0.57165357490759649296e-3_f64 * t16089 * t19707 - 0.22866142996303859718e-2_f64 * t12013 * t6308 - 0.57165357490759649296e-3_f64 * t4834 * t4803 + 0.47637797908966374413e-3_f64 * t4834 * t4808 - 0.42874018118069736972e-3_f64 * t4899 * t19718 + 0.21437009059034868486e-3_f64 * t16067 * t19722 + 0.28582678745379824648e-3_f64 * t4892 * t19726;
    t19729
}
