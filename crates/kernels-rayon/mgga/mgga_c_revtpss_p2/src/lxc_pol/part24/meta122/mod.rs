//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta122 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk659;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk660;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta122(t1549: f64, t2703: f64, t1568: f64, t213: f64, t1580: f64, t779: f64, t689: f64, t1579: f64, t72: f64, t686: f64, t2465: f64, t1558: f64, t251: f64, t231: f64, t2783: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4455, t4474, t4477, t4478, t4480, t4481, t4482, t4494) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk659(t1549, t2703, t1568, t213, t1580, t779, t689, t1579, t72, t686, t2465, t1558, t251);
        let t4496 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk660(t231, t2783, t4494);
    (t4455, t4474, t4477, t4478, t4480, t4481, t4482, t4494, t4496)
}
