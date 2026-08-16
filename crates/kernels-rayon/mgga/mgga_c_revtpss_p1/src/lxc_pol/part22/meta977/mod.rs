//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta977 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3286;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta977(t162: f64, t4403: f64, t50903: f64, t50089: f64, t14331: f64, t13312: f64, t4401: f64, t4402: f64, t50880: f64, t50883: f64, t50888: f64, t2609: f64, t5944: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t62290, t62293, t62296, t62297, t62298, t62299, t62300) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3286(t162, t4403, t50903, t50089, t14331, t13312, t4401, t4402, t50880, t50883, t50888, t2609, t5944);
    (t62290, t62293, t62296, t62297, t62298, t62299, t62300)
}
