//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta277 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1233;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1234;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1235;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1236;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1237;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta277(t1927: f64, t8143: f64, t2122: f64, t7719: f64, t5: f64, t1923: f64, t2123: f64, t7566: f64, t7702: f64, t7706: f64, t7709: f64, t117: f64, t30: f64, t265: f64, t393: f64, t1518: f64, t2163: f64, t7855: f64, t1469: f64, t2129: f64, t45: f64, t7794: f64, t1479: f64, t343: f64, t136: f64, t1785: f64, t2138: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t1802: f64, t2137: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t8144 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1233(t1927, t8143);
        let t8147 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1234(t2122, t7719);
        let (t8151, t8152) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1235(t5, t1923, t2123, t7566, t7702, t7706, t7709, t8144, t8147, t117);
        let (t8158, t8161, t8166, t8171, t8172, t8177) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1236(t30, t265, t393, t1518, t2163, t7855, t1469, t2129, t45, t7794, t1479, t343, t136, t1785, t2138, dens_threshold, rho0, zeta_threshold);
        let t8184 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1237(t1802, t2137);
    (t8144, t8147, t8151, t8152, t8158, t8161, t8166, t8171, t8172, t8177, t8184)
}
