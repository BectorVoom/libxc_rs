//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta522 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1551;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1552;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta522(t1247: f64, t24772: f64, t3172: f64, t20819: f64, t5292: f64, t17505: f64, t20783: f64, t1260: f64, t24699: f64, t21242: f64, t5378: f64, t1785: f64, t21271: f64, t1261: f64, t24248: f64, t247: f64, t3634: f64, t21233: f64, t5381: f64, t17401: f64, t20926: f64, t24770: f64, t73: f64, t12916: f64, t24752: f64, t3718: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t82553, t82555, t82560, t82565, t82595, t82597) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1551(t1247, t24772, t3172, t20819, t5292, t17505, t20783, t1260, t24699, t21242, t5378, t1785, t21271);
        let (t82603, t82656, t82678, t82725, t82749) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1552(t1261, t24248, t247, t3634, t21233, t5381, t17401, t20926, t24770, t73, t12916, t24752, t3718);
    (t82553, t82555, t82560, t82565, t82595, t82597, t82603, t82656, t82678, t82725, t82749)
}
