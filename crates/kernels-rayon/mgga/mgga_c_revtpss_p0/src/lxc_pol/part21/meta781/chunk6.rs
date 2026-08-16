//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2799/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2799(t14587: f64, t2782: f64, t39608: f64, t10069: f64, t14496: f64, t10639: f64, t10657: f64, t14546: f64, t39712: f64, t39719: f64, t39723: f64, t39724: f64, t39726: f64, t40284: f64, t4424: f64, t4494: f64, t4514: f64, t51375: f64, t820: f64, t836: f64, t837: f64) -> f64 {
    let t51460 = t2782 * t39608 * t14587;
    let t51470 = t10069 * t14496;
    let t51471 = 0.21951497276451705329e-1_f64 * t51470;
    let t51479 = -0.32927245914677557992e-1_f64 * t39712 + 0.58911598146606471822e-3_f64 * t39719 - t39723 - 0.65854491829355115984e-1_f64 * t51460 + 0.7805952431506226415e-2_f64 * t39724 - 0.21951497276451705329e-1_f64 * t39726 - 0.19756347548806534796e1_f64 * t820 * t10657 * t4424 - 0.39512695097613069591e1_f64 * t4514 * t51375 * t837 - t51471 - 0.11853808529283920877e2_f64 * t14546 * t4494 * t40284 * t836 - 0.65854491829355115987e0_f64 * t4514 * t4494 * t10639;
    t51479
}
