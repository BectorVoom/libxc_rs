//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta617 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2124;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2125;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta617(t28189: f64, t7235: f64, t2014: f64, t7900: f64, t94358: f64, t13716: f64, t1450: f64, t7237: f64, t18163: f64, t7735: f64, t27137: f64, t4254: f64, t25082: f64, t75353: f64, t8717: f64, t7311: f64, t9593: f64, t28196: f64, t28198: f64, t28166: f64, t7234: f64, t28168: f64, t27153: f64, t32113: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t98559, t98562, t98567, t98569, t98571) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2124(t28189, t7235, t2014, t7900, t94358, t13716, t1450, t7237, t18163, t7735, t27137, t4254);
        let (t98574, t98578, t98581, t98584) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2125(t25082, t75353, t8717, t7311, t9593, t28196, t28198, t28166, t7234, t28168, t27153, t32113);
    (t98559, t98562, t98567, t98569, t98571, t98574, t98578, t98581, t98584)
}
