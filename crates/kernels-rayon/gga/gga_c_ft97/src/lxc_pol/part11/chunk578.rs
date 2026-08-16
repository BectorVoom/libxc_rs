//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 578/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk578(t1599: f64, t1617: f64, t1618: f64, t1624: f64, t1665: f64, t372: f64, t399: f64, t534: f64, t64: f64, t7899: f64, t7906: f64, t8000: f64, t8001: f64, t8003: f64, t8009: f64, t8011: f64, t8015: f64, t8018: f64, t8032: f64, t8036: f64, t8039: f64, t8042: f64, t8044: f64, t8047: f64, t8052: f64, t8053: f64) -> f64 {
    let t8057 = -0.49022040019937983366e-5_f64 * t8000 * t8001 * t8003 + 0.13774561697978600408e-4_f64 * t8009 * t8011 + 0.41352194951222972388e-4_f64 * t8015 * t8011 + 0.13094861734553941256e-2_f64 * t1617 * t1618 * t8018 + 0.35564283887055077925e-1_f64 * t1665 * t399 - 0.40559281352147498558e-4_f64 * t7906 * t7899 * t1599 - 0.33776098467676728323e-5_f64 * t534 * t7899 * t1599 + 0.58097170218823199823e-3_f64 * t372 * t8032 - 0.58097170218823199822e-3_f64 * t1624 * t8036 + 0.69764702839313376e-2_f64 * t372 * t8039 - 0.69764702839313376e-1_f64 * t8042 * t8044 - 0.69764702839313376e-2_f64 * t1624 * t8047 - 6.0_f64 * t64 * t8052 * t8053;
    t8057
}
