//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3488/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3488(t16163: f64, t4879: f64, t1063: f64, t19681: f64, t3172: f64, t11710: f64, t19625: f64, t4899: f64, t19687: f64, t3160: f64, t65338: f64, t11672: f64, t11675: f64, t11994: f64, t15963: f64, t1671: f64, t19501: f64, t19682: f64, t19702: f64, t19778: f64, t19782: f64, t3092: f64, t3164: f64, t3188: f64, t42391: f64, t4783: f64, t54144: f64, t54471: f64, t6263: f64) -> f64 {
    let t65627 = t4879 * t16163;
    let t65630 = t1063 * t3172 * t19681;
    let t65637 = t4899 * t11710 * t19625;
    let t65650 = t1063 * t3172 * t19687;
    let t65654 = t65338 * t3160;
    let t65659 = 0.57165357490759649296e-3_f64 * t65627 - 0.3811023832717309953e-3_f64 * t65630 - 0.30488190661738479624e-2_f64 * t54471 * t4783 - 0.30488190661738479624e-2_f64 * t11672 * t19778 - 0.19055119163586549765e-3_f64 * t65637 + 0.28582678745379824648e-3_f64 * t4899 * t3092 * t19501 * t15963 + 0.47637797908966374414e-3_f64 * t11675 * t19782 + 0.14481890564325777821e-1_f64 * t54144 * t1671 - 0.28582678745379824648e-3_f64 * t42391 * t6263 + 0.31758531939310916276e-3_f64 * t65650 - 0.57165357490759649296e-3_f64 * t3188 * t19682 - 0.21437009059034868486e-3_f64 * t65654 * t3164 - 0.28582678745379824648e-3_f64 * t11994 * t19702;
    t65659
}
