//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1314/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1314(t12210: f64, t14121: f64, t11426: f64, t50998: f64, t53447: f64, t11430: f64, t12213: f64, t14747: f64, t15320: f64, t2408: f64, t2409: f64, t3066: f64, t335: f64, t338: f64, t3907: f64, t4053: f64, t51651: f64, t53730: f64, t56776: f64, t56783: f64, t56787: f64, t56791: f64, t56793: f64, t56799: f64, t56811: f64, t56813: f64, t6781: f64) -> f64 {
    let t56815 = t14121 * t12210;
    let t56818 = t50998 * t53447 * t11426;
    let t56821 = t50998 * t53447 * t11430;
    let t56823 = t56776 / 24.0_f64 - t335 * t338 * t3907 * t4053 / 96.0_f64 - t53730 - t56783 / 48.0_f64 + t56787 / 1536.0_f64 - t56791 / 384.0_f64 - t56793 / 96.0_f64 + t56799 / 48.0_f64 + t2408 * t2409 * t6781 * t15320 / 24.0_f64 + t3066 * t2409 * t12213 * t14747 / 24.0_f64 - 35.0_f64 / 216.0_f64 * t51651 - t56811 / 768.0_f64 + t56813 / 24.0_f64 + t56815 / 8.0_f64 + t56818 / 192.0_f64 + t56821 / 192.0_f64;
    t56823
}
