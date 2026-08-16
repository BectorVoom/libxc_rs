//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta117 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk592;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta117(t290: f64, t2846: f64, t941: f64, t945: f64, t307: f64, t944: f64, t302: f64, t2904: f64, t310: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2925, t2926, t2930, t2938, t2942, t2943, t2950, t2957, t2966, t2967, t2968, t2969) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk592(t290, t2846, t941, t945, t307, t944, t302, t2904, t310);
    (t2925, t2926, t2930, t2938, t2942, t2943, t2950, t2957, t2966, t2967, t2968, t2969)
}
