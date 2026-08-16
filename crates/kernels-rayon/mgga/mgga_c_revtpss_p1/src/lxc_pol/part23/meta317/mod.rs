//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta317 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1606;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta317(t1514: f64, t2289: f64, t4264: f64, t625: f64, t4288: f64, t2339: f64, t4287: f64, t2349: f64, t97: f64, t105: f64, t2357: f64, t1468: f64, t9335: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t13448, t13451, t13453, t13458, t13475, t13496, t13550) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1606(t1514, t2289, t4264, t625, t4288, t2339, t4287, t2349, t97, t105, t2357, t1468, t9335);
    (t13448, t13451, t13453, t13458, t13475, t13496, t13550)
}
