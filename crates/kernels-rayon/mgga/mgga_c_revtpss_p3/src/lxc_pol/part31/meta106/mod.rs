//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta106 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk646;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta106(t2435: f64, t2439: f64, t2502: f64, t2504: f64, t2509: f64, t2511: f64, t730: f64, t722: f64, t164: f64, t172: f64, t2538: f64, t123: f64, t147: f64, t2434: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2548, t2549, t2552, t2553, t2554, t2555, t2556, t2557, t2562) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk646(t2435, t2439, t2502, t2504, t2509, t2511, t730, t722, t164, t172, t2538, t123, t147, t2434);
    (t2548, t2549, t2552, t2553, t2554, t2555, t2556, t2557, t2562)
}
