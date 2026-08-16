//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2217/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2217(t1479: f64, t2282: f64, t101303: f64, t101376: f64, t13312: f64, t13392: f64, t13396: f64, t15936: f64, t1923: f64, t1927: f64, t2122: f64, t2123: f64, t2251: f64, t2258: f64, t25117: f64, t25146: f64, t25150: f64, t26776: f64, t26783: f64, t26786: f64, t26789: f64, t29355: f64, t29363: f64, t29364: f64, t29367: f64, t6954: f64, t6977: f64, t72: f64, t7571: f64, t7702: f64, t8143: f64, t8144: f64, t8147: f64, t92612: f64, t96733: f64) -> f64 {
    let t104379 = t1479 * t2282;
    let t104403 = t25117 * t8147 / 3.0_f64 + t101376 * t2123 / 3.0_f64 - t1923 * t2122 * t101303 / 6.0_f64 - t7702 * t26783 / 6.0_f64 - t7702 * t26786 / 3.0_f64 - t7702 * t26789 / 6.0_f64 - t25150 * t8144 / 6.0_f64 - t6954 * t29364 / 3.0_f64 - t6954 * t29367 / 3.0_f64 - t1923 * (-20.0_f64 / 27.0_f64 * t104379 * t2251 + 20.0_f64 / 9.0_f64 * t29355 * t2258 + 5.0_f64 / 108.0_f64 * t96733 * t15936 + 5.0_f64 / 9.0_f64 * t26776 * t13396 + 5.0_f64 / 18.0_f64 * t26776 * t13392 - 5.0_f64 / 6.0_f64 * t7571 * t13312 + t92612) * t72 * t1927 / 6.0_f64 - t1923 * t29363 * t6977 / 3.0_f64 - t1923 * t8143 * t25146 / 6.0_f64;
    t104403
}
