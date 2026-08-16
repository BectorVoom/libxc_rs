//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta341 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1349;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta341(t11239: f64, t1243: f64, t460: f64, t3596: f64, t13038: f64, t1275: f64, t225: f64, t10270: f64, t10272: f64, t10279: f64, t10281: f64, t10288: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13126, t13127, t13141, t13142, t13147, t13148, t13182, t13261, t13262, t13263, t13264, t13265) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1349(t11239, t1243, t460, t3596, t13038, t1275, t225, t10270, t10272, t10279, t10281, t10288);
    (t13126, t13127, t13141, t13142, t13147, t13148, t13182, t13261, t13262, t13263, t13264, t13265)
}
