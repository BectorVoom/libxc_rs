//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta171 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk823;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta171(t3682: f64, t461: f64, t1226: f64, t140: f64, t1222: f64, t1225: f64, t2258: f64, t1012: f64, t1224: f64, t3367: f64, t2251: f64, t1121: f64, t404: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3684, t3685, t3686, t3688, t3689, t3693, t3694, t3698) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk823(t3682, t461, t1226, t140, t1222, t1225, t2258, t1012, t1224, t3367, t2251, t1121, t404);
    (t3684, t3685, t3686, t3688, t3689, t3693, t3694, t3698)
}
