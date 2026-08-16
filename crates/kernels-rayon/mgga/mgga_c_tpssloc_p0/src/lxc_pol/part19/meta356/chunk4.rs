//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1291/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1291(t41654: f64, t41961: f64, t41937: f64, t41940: f64, t41943: f64, t41945: f64, t41948: f64, t41951: f64, t41954: f64, t41957: f64, t41964: f64, t41967: f64, t41970: f64, t41973: f64) -> f64 {
    let t42086 = 0.31003950617283950618e1_f64 * t41654;
    let t42087 = 0.13388493827160493828e1_f64 * t41961;
    let t42092 = -0.3560484375e1_f64 * t41937 - 0.28483875e1_f64 * t41940 + 0.1151859375e0_f64 * t41943 + 0.46074375e0_f64 * t41945 - 0.379785e1_f64 * t41948 + 0.614325e0_f64 * t41951 + 0.85451625e1_f64 * t41954 - 0.46074375e0_f64 * t41957 + t42086 + t42087 - 0.10954222222222222222e0_f64 * t41964 - 0.21908444444444444444e0_f64 * t41967 - 0.295764e1_f64 * t41970 + 0.65725333333333333332e0_f64 * t41973;
    t42092
}
