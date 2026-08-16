//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta635 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2562;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta635(t3531: f64, t6556: f64, t6552: f64, t3362: f64, t5825: f64, t606: f64, t3417: f64, t141: f64, t1121: f64, t18281: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t20261, t20263, t20265, t20266, t20267, t20268, t20272) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2562(t3531, t6556, t6552, t3362, t5825, t606, t3417, t141, t1121, t18281);
    (t20261, t20263, t20265, t20266, t20267, t20268, t20272)
}
