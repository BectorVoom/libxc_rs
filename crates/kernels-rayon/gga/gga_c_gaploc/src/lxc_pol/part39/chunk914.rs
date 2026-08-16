//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 914/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk914(t10525: f64, t10526: f64, t41749: f64, t40106: f64, t40109: f64, t1445: f64, t3085: f64, t574: f64, t7980: f64, t3149: f64, t8072: f64, t3153: f64, t8063: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41752 = 0.42900587942220512002e1_f64 * t10525 * t10526 * t41749;
    let t41753 = 0.29792074959875355558e-1_f64 * t40106;
    let t41754 = 0.59584149919750711116e-1_f64 * t40109;
    let t41759 = 0.92023022289409799224e1_f64 * t574 * t1445 * t7980 * t3085;
    let t41761 = 0.35750489951850426669e0_f64 * t3149 * t8072;
    let t41767 = 0.23833659967900284446e0_f64 * t3153 * t8063;
    (t41752, t41753, t41754, t41759, t41761, t41767)
}
