//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 862/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk862(t1608: f64, t1630: f64, t7998: f64, t1632: f64, t39: f64, t8003: f64, t395: f64, t45: f64, t44: f64, t52: f64, t54: f64, t5588: f64) -> (f64, f64, f64, f64) {
    let t37668 = t1608 * t7998 * t1630;
    let t37670 = t1632 * t39 * t8003;
    let t37678 = 1.0_f64 / t45 / t395;
    let t37685 = t52 * t54 / t44 / t5588;
    (t37668, t37670, t37678, t37685)
}
