//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta347 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1834;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta347(t11710: f64, t3096: f64, t3091: f64, t1020: f64, t3105: f64, t247: f64, t2862: f64, t3109: f64, t1063: f64, t126: f64, t3181: f64, t2853: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t11711, t11712, t11714, t11722, t11723, t11725, t11727) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1834(t11710, t3096, t3091, t1020, t3105, t247, t2862, t3109, t1063, t126, t3181, t2853);
    (t11711, t11712, t11714, t11722, t11723, t11725, t11727)
}
