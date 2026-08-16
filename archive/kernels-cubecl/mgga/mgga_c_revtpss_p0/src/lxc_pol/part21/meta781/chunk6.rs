//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2799/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2799<F: Float>(t14587: F, t2782: F, t39608: F, t10069: F, t14496: F, t10639: F, t10657: F, t14546: F, t39712: F, t39719: F, t39723: F, t39724: F, t39726: F, t40284: F, t4424: F, t4494: F, t4514: F, t51375: F, t820: F, t836: F, t837: F) -> F {
    let t51460 = t2782 * t39608 * t14587;
    let t51470 = t10069 * t14496;
    let t51471 = F::cast_from(0.21951497276451705329e-1_f64) * t51470;
    let t51479 = -F::cast_from(0.32927245914677557992e-1_f64) * t39712 + F::cast_from(0.58911598146606471822e-3_f64) * t39719 - t39723 - F::cast_from(0.65854491829355115984e-1_f64) * t51460 + F::cast_from(0.7805952431506226415e-2_f64) * t39724 - F::cast_from(0.21951497276451705329e-1_f64) * t39726 - F::cast_from(0.19756347548806534796e1_f64) * t820 * t10657 * t4424 - F::cast_from(0.39512695097613069591e1_f64) * t4514 * t51375 * t837 - t51471 - F::cast_from(0.11853808529283920877e2_f64) * t14546 * t4494 * t40284 * t836 - F::cast_from(0.65854491829355115987e0_f64) * t4514 * t4494 * t10639;
    t51479
}
