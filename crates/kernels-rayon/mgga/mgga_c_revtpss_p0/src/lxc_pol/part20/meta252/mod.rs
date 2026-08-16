//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta252 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1085;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta252(t2923: f64, t910: f64, t2927: f64, t287: f64, t2922: f64, t275: f64, t2875: f64, t934: f64, t2926: f64, t11132: f64, t11134: f64, t11136: f64, t11138: f64, t11140: f64, t11147: f64, t11153: f64, t11158: f64, t11162: f64, t11167: f64, t11171: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11294, t11296, t11298, t11299, t11300, t11301, t11303, t11315) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1085(t2923, t910, t2927, t287, t2922, t275, t2875, t934, t2926, t11132, t11134, t11136, t11138, t11140, t11147, t11153, t11158, t11162, t11167, t11171);
    (t11294, t11296, t11298, t11299, t11300, t11301, t11303, t11315)
}
