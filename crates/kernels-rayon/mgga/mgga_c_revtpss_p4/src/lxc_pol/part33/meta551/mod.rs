//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta551 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1935;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1936;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta551(t7898: f64, t7901: f64, t4248: f64, t7742: f64, t28172: f64, t7900: f64, t2014: f64, t2034: f64, t22483: f64, t30: f64, t5966: f64, t1963: f64, t1544: f64, t1583: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t29580, t29582, t29583, t29585, t29589, t29590, t29591, t29592) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1935(t7898, t7901, t4248, t7742, t28172, t7900, t2014, t2034, t22483, t30, t5966, t1963);
        let t29598 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1936(t1544, t1583);
    (t29580, t29582, t29583, t29585, t29589, t29590, t29591, t29592, t29598)
}
