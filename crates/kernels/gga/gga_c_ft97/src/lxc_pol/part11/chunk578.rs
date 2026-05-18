//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 578/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk578<F: Float>(t1599: F, t1617: F, t1618: F, t1624: F, t1665: F, t372: F, t399: F, t534: F, t64: F, t7899: F, t7906: F, t8000: F, t8001: F, t8003: F, t8009: F, t8011: F, t8015: F, t8018: F, t8032: F, t8036: F, t8039: F, t8042: F, t8044: F, t8047: F, t8052: F, t8053: F) -> F {
    let t8057 = -F::new(0.49022040019937983366e-5) * t8000 * t8001 * t8003 + F::new(0.13774561697978600408e-4) * t8009 * t8011 + F::new(0.41352194951222972388e-4) * t8015 * t8011 + F::new(0.13094861734553941256e-2) * t1617 * t1618 * t8018 + F::new(0.35564283887055077925e-1) * t1665 * t399 - F::new(0.40559281352147498558e-4) * t7906 * t7899 * t1599 - F::new(0.33776098467676728323e-5) * t534 * t7899 * t1599 + F::new(0.58097170218823199823e-3) * t372 * t8032 - F::new(0.58097170218823199822e-3) * t1624 * t8036 + F::new(0.69764702839313376e-2) * t372 * t8039 - F::new(0.69764702839313376e-1) * t8042 * t8044 - F::new(0.69764702839313376e-2) * t1624 * t8047 - F::new(6.0) * t64 * t8052 * t8053;
    t8057
}
