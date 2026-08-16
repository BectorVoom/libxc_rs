//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta321 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1319;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1320;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta321(t755: f64, t9586: f64, t2619: f64, t2622: f64, t2390: f64, t72: f64, t757: f64, t2629: f64, t9863: f64, t123: f64, t752: f64, t2630: f64, t9866: f64, t9575: f64, t9572: f64, t177: f64, t762: f64, t760: f64, t9419: f64, t2516: f64, t2523: f64, t9387: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10568, t10569, t10574, t10577, t10579) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1319(t755, t9586, t2619, t2622, t2390, t72, t757, t2629, t9863, t123, t752, t2630);
        let (t10582, t10584, t10586, t10588, t10592, t10593, t10596) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1320(t2629, t9866, t9575, t9572, t177, t2390, t762, t760, t9419, t2516, t2523, t9387);
    (t10568, t10569, t10574, t10577, t10579, t10582, t10584, t10586, t10588, t10592, t10593, t10596)
}
