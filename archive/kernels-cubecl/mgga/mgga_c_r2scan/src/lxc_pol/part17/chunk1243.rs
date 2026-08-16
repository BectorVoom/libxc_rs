//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1243/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1243<F: Float>(t40220: F, t41743: F, t41748: F, t41749: F, t43631: F, t43635: F, t43638: F, t43641: F, t43643: F, t43645: F, t43648: F, t43650: F) -> F {
    let t44483 = F::cast_from(0.21951497276451705328e0_f64) * t43631 - t41743 - F::cast_from(0.43902994552903410656e0_f64) * t43635 - F::cast_from(0.5200933044032561138e0_f64) * t43638 - F::cast_from(0.20803732176130244552e1_f64) * t43641 - F::cast_from(0.95219938395347901947e-2_f64) * t43643 - F::cast_from(0.28565981518604370584e-1_f64) * t43645 + F::cast_from(0.26198215989259945076e-1_f64) * t43648 + F::cast_from(0.17336443480108537126e0_f64) * t43650 - t41748 - t41749 + F::cast_from(0.90044238659382329742e0_f64) * t40220;
    t44483
}
