//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta622 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2072;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta622(t25387: f64, t99349: f64, t2470: f64, t27340: f64, t7063: f64, t99271: f64, t7060: f64, t136: f64, t2457: f64, t7778: f64, t25299: f64, t25412: f64, t99348: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t99351, t99365, t99366, t99375, t99380, t99381, t99389) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2072(t25387, t99349, t2470, t27340, t7063, t99271, t7060, t136, t2457, t7778, t25299, t25412, t99348);
    (t99351, t99365, t99366, t99375, t99380, t99381, t99389)
}
