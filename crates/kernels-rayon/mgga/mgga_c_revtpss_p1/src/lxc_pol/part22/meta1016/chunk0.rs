//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3508/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3508(t11675: f64, t19785: f64, t1043: f64, t1045: f64, t15145: f64, t15691: f64, t15700: f64, t15895: f64, t15957: f64, t16017: f64, t16226: f64, t19501: f64, t19741: f64, t19776: f64, t19934: f64, t19998: f64, t3091: f64, t3092: f64, t3155: f64, t3188: f64, t42580: f64, t43175: f64, t4583: f64, t4892: f64, t53800: f64, t53993: f64, t53998: f64, t54026: f64, t55100: f64, t6266: f64) -> f64 {
    let t66261 = t11675 * t19785;
    let t66263 = -0.47637797908966374413e-4_f64 * t42580 + 0.3811023832717309953e-3_f64 * t53993 + 0.28582678745379824648e-3_f64 * t3091 * t3092 * t54026 * t6266 + 0.57165357490759649296e-3_f64 * t3091 * t3092 * t15957 * t19776 + 0.11433071498151929859e-2_f64 * t53998 - 0.11433071498151929859e-2_f64 * t3188 * t19934 - 0.85748036236139473944e-3_f64 * t19741 * t16017 - 0.85748036236139473944e-3_f64 * t53800 * t15895 - 0.11433071498151929859e-2_f64 * t15700 * t15691 * t1045 * t15145 + 0.11433071498151929859e-2_f64 * t16226 * t15691 * t3155 * t4583 * t1043 - 0.60976381323476959249e-2_f64 * t55100 * t19998 - 0.57165357490759649296e-3_f64 * t4892 * t3092 * t19501 * t43175 + 0.3811023832717309953e-3_f64 * t66261;
    t66263
}
