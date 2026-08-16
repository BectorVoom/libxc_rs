//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta199 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk826;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta199(t45: f64, t57: f64, t1469: f64, t2375: f64, t4186: f64, t606: f64, t78: f64, t2382: f64, t81: f64, t162: f64, t187: f64, t150: f64, t190: f64, t1532: f64, t750: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4377, t4384, t4391, t4392, t4394, t4395, t4396, t4397) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk826(t45, t57, t1469, t2375, t4186, t606, t78, t2382, t81, t162, t187, t150, t190, t1532, t750, zeta_threshold);
    (t4377, t4384, t4391, t4392, t4394, t4395, t4396, t4397)
}
