//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1157/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1157(t10530: f64, t1882: f64, t10522: f64, t10526: f64, t10700: f64, t2846: f64, t8232: f64, t313: f64, t41743: f64, t89: f64, t295: f64, t41752: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t44393 = t1882 * t10530;
    let t44395 = t1882 * t10522;
    let t44397 = t1882 * t10526;
    let t44426 = t1882 * t10700;
    let t44428 = t8232 * t2846;
    let t44436 = 280.0_f64 / 243.0_f64 * t89 * t41743 * t313;
    let t44445 = t41752 * t295;
    (t44393, t44395, t44397, t44426, t44428, t44436, t44445)
}
