//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta468 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1725;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1726;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1727;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1728;
use chunk4::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1729;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta468(t25231: f64, t25242: f64, t25253: f64, t25275: f64, t25283: f64, t25251: f64, t25256: f64, t25258: f64, t25263: f64, t25267: f64, t25271: f64, t25278: f64, t25280: f64, t25223: f64, t25225: f64, t25229: f64, t25235: f64, t25238: f64, t25246: f64, t25248: f64, t26450: f64, t233: f64, t1957: f64, t122: f64, t2061: f64, t72: f64, t25412: f64, t25411: f64, t2466: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t26454, t26457, t26462, t26468, t26471, t26472) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1725(t25231, t25242, t25253, t25275, t25283, t25251, t25256, t25258, t25263, t25267, t25271, t25278, t25280);
        let t26473 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1726(t25223, t25225, t25229, t25235, t25238, t25246, t25248, t26450, t26454, t26457, t26472);
        let (t26474, t26475, t26481) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1727(t233, t26473, t1957, t122, t2061, t72);
        let t26482 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1728(t25412, t26481);
        let (t26483, t26485) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1729(t25411, t26482, t2466, t26481);
    (t26454, t26457, t26462, t26468, t26471, t26473, t26474, t26475, t26481, t26482, t26483, t26485)
}
