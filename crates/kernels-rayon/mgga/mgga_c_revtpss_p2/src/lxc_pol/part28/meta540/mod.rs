//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta540 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1989;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta540(t4147: f64, t7311: f64, t1925: f64, t36: f64, t606: f64, t7933: f64, t1450: f64, t11239: f64, t3268: f64, t211: f64, t9644: f64, t138: f64, t785: f64, t9302: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t32113, t32592, t33651, t35070, t36870, t39643, t40270) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1989(t4147, t7311, t1925, t36, t606, t7933, t1450, t11239, t3268, t211, t9644, t138, t785, t9302);
    (t32113, t32592, t33651, t35070, t36870, t39643, t40270)
}
