//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta813 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2658;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta813(t19021: f64, t3011: f64, t19045: f64, t300: f64, t379: f64, t4746: f64, t1679: f64, t3057: f64, t1078: f64, t6244: f64, t1678: f64, t4743: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t64504, t64510, t64547, t64550, t64555, t64605) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2658(t19021, t3011, t19045, t300, t379, t4746, t1679, t3057, t1078, t6244, t1678, t4743);
    (t64504, t64510, t64547, t64550, t64555, t64605)
}
