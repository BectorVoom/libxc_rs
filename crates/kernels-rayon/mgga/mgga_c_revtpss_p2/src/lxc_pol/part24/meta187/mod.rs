//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta187 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk908;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk909;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta187(t1337: f64, t9586: f64, t4146: f64, t565: f64, t1333: f64, t3860: f64, t30: f64, t513: f64, t33: f64, t516: f64, t3896: f64, t9303: f64, t784: f64, t209: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9588, t9593, t9598, t9603, t9605, t9615, t9617, t9639) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk908(t1337, t9586, t4146, t565, t1333, t3860, t30, t513, t33, t516, t3896, t9303);
        let (t9644, t9645, t9646) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk909(t784, t209);
    (t9588, t9593, t9598, t9603, t9605, t9615, t9617, t9639, t9644, t9645, t9646)
}
