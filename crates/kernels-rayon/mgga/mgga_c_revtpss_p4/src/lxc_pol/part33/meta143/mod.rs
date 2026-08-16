//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta143 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk765;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta143(t1330: f64, t72: f64, t757: f64, t530: f64, t566: f64, t525: f64, t527: f64, t2608: f64, t520: f64, t512: f64, t19: f64, t27: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3825, t3826, t3828, t3833, t3841, t3853, t3854, t3857) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk765(t1330, t72, t757, t530, t566, t525, t527, t2608, t520, t512, t19, t27);
    (t3825, t3826, t3828, t3833, t3841, t3853, t3854, t3857)
}
