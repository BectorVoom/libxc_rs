//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta29 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk202;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk203;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk204;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk205;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk206;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta29(t546: f64, t555: f64, t213: f64, t556: f64, t149: f64, t198: f64, t522: f64, t524: f64, t532: f64, t118: f64, t508: f64, t511: f64, param_d: f64, t116: f64, t117: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t557, t560, t561) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk202(t546, t555, t213);
        let (t562, t565, t566) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk203(t556, t561, t213);
        let t569 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk204(t149, t198, t522, t524, t532, t566);
        let (t571, t572) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk205(t118, t508, t511, t569, param_d);
        let t573 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk206(t116, t117);
    (t557, t560, t561, t562, t565, t566, t569, t571, t572, t573)
}
