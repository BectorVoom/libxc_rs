//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1224/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1224(t39958: f64, t39962: f64, t39964: f64, t39967: f64, t39969: f64, t39977: f64, t39979: f64, t39984: f64, t39972: f64, t39975: f64, t39982: f64, t39987: f64) -> f64 {
    let t41634 = 0.18629842481251516498e0_f64 * t39958;
    let t41635 = 0.28565981518604370584e-1_f64 * t39962;
    let t41636 = 0.95219938395347901946e-2_f64 * t39964;
    let t41637 = 0.95219938395347901946e-2_f64 * t39967;
    let t41638 = 0.28565981518604370584e-1_f64 * t39969;
    let t41641 = 0.42683466926433871473e0_f64 * t39977;
    let t41642 = 0.21951497276451705328e-1_f64 * t39979;
    let t41644 = 0.10975748638225852664e-1_f64 * t39984;
    let t41646 = t41634 + t41635 + t41636 + t41637 + t41638 + 0.5200933044032561138e0_f64 * t39972 + 0.20803732176130244552e1_f64 * t39975 - t41641 - t41642 - 0.92461031893912198007e0_f64 * t39982 + t41644 + 0.26198215989259945076e-1_f64 * t39987;
    t41646
}
