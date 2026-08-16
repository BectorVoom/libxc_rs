//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta582 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2002;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta582(t11735: f64, t1968: f64, t11772: f64, t25515: f64, t3114: f64, t3223: f64, t7131: f64, t11273: f64, t25504: f64, t25508: f64, t11263: f64, t7122: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t93750, t93751, t93752, t93764, t93783, t93796, t93801) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2002(t11735, t1968, t11772, t25515, t3114, t3223, t7131, t11273, t25504, t25508, t11263, t7122);
    (t93750, t93751, t93752, t93764, t93783, t93796, t93801)
}
