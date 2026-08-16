//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta526 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1854;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta526(t7063: f64, t94878: f64, t25877: f64, t94801: f64, t1419: f64, t786: f64, t2453: f64, t25949: f64, t25898: f64, t112: f64, t843: f64, t239: f64, t655: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t94879, t94886, t94890, t94894, t94913, t94921, t94973, t94975) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1854(t7063, t94878, t25877, t94801, t1419, t786, t2453, t25949, t25898, t112, t843, t239, t655);
    (t94879, t94886, t94890, t94894, t94913, t94921, t94973, t94975)
}
