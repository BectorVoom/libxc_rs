//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1257/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1257(t15609: f64, t4893: f64, t3117: f64, t4583: f64, t4786: f64, t3092: f64, t3090: f64, t4954: f64, t4757: f64, t11264: f64, t11271: f64, t11774: f64, t11859: f64, t11875: f64, t11927: f64, t15583: f64, t15586: f64, t15592: f64, t15596: f64, t15601: f64, t15606: f64, t3091: f64, t3097: f64) -> f64 {
    let t15610 = t4893 * t15609;
    let t15611 = t3117 * t15610;
    let t15614 = t4583 * t4786;
    let t15615 = t3092 * t15614;
    let t15618 = t4954 * t3090;
    let t15621 = t4757 * t4786;
    let t15622 = t3117 * t15621;
    let t15625 = -t15583 - 0.28582678745379824648e-3_f64 * t11774 * t15586 - 0.95275595817932748826e-4_f64 * t11264 - 0.15244095330869239812e-2_f64 * t11271 + 0.14291339372689912324e-3_f64 * t3091 * t15592 + 0.23818898954483187207e-3_f64 * t3091 * t15596 + 0.14291339372689912324e-3_f64 * t3091 * t15601 + 0.42874018118069736972e-3_f64 * t11875 * t15606 - 0.85748036236139473944e-3_f64 * t11859 * t15611 + 0.28582678745379824648e-3_f64 * t3091 * t15615 + 0.28582678745379824648e-3_f64 * t15618 * t3097 + 0.85748036236139473944e-3_f64 * t11927 * t15622;
    t15625
}
