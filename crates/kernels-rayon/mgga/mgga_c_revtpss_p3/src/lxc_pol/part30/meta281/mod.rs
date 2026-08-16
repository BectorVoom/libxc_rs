//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta281 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1233;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1234;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1235;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta281(t572: f64, t7953: f64, t1469: f64, t1479: f64, t61: f64, t6971: f64, t7571: f64, t72: f64, t1927: f64, t2122: f64, t7719: f64, t5: f64, t265: f64, t393: f64, t1923: f64, t2123: f64, t7566: f64, t7702: f64, t7706: f64, t7709: f64, t117: f64, t1518: f64, t2163: f64, t7855: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7955, t8142, t8143, t8144) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1233(t572, t7953, t1469, t1479, t61, t6971, t7571, t72, t1927);
        let t8147 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1234(t2122, t7719);
        let (t8151, t8152, t8158, t8161) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1235(t5, t265, t393, t1923, t2123, t7566, t7702, t7706, t7709, t8144, t8147, t117, t1518, t2163, t7855);
    (t7955, t8142, t8143, t8144, t8147, t8151, t8152, t8158, t8161)
}
