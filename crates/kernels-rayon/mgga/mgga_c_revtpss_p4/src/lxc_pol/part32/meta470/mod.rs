//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta470 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1697;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1698;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1699;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta470(t2097: f64, t785: f64, t1358: f64, t2439: f64, t2435: f64, t7493: f64, t26069: f64, t26277: f64, t26072: f64, t7515: f64, t116: f64, t7356: f64, t2106: f64, t4147: f64, t531: f64, t7535: f64, t198: f64, t206: f64, t2070: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t26358, t26359, t26361, t26363, t26365, t26366, t26399) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1697(t2097, t785, t1358, t2439, t2435, t7493, t26069, t26277, t26072, t7515, t116, t7356);
        let t26405 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1698(t2106, t4147);
        let (t26411, t26425) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1699(t531, t7535, t198, t206, t2070);
    (t26358, t26359, t26361, t26363, t26365, t26366, t26399, t26405, t26411, t26425)
}
