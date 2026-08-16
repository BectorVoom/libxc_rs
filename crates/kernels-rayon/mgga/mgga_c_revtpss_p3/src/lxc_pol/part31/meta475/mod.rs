//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta475 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1742;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1743;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1744;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1745;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta475(t1927: f64, t644: f64, t1926: f64, t531: f64, t7311: f64, t1962: f64, t198: f64, t206: f64, t2411: f64, t30: f64, t1946: f64, t2684: f64, t7043: f64, t820: f64, t843: f64, t857: f64, t240: f64, t7036: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t25163, t25164, t25190, t25206) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1742(t1927, t644, t1926, t531, t7311, t1962, t198, t206);
        let t25207 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1743(t2411, t30);
        let (t25220, t25222) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1744(t1946, t2684, t7043, t820, t843);
        let (t25223, t25227) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1745(t25222, t857, t240, t7036);
    (t25163, t25164, t25190, t25206, t25207, t25220, t25222, t25223, t25227)
}
