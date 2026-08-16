//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2860/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2860(t41658: f64, t41675: f64, t41684: f64, t59655: f64, t59657: f64, t59661: f64, t59663: f64, t59665: f64, t59670: f64, t59674: f64, t59678: f64, t59680: f64, t59684: f64) -> f64 {
    let t59802 = -0.50735802469135802469e-2_f64 * t41658 + 0.15220740740740740741e-1_f64 * t41675 + 0.35515061728395061728e-1_f64 * t41684 - 0.41096e0_f64 * t59655 - 0.50735802469135802467e-2_f64 * t59657 + 0.41096e0_f64 * t59661 - 0.2283111111111111111e-1_f64 * t59663 + 0.76103703703703703701e-2_f64 * t59665 - 0.2283111111111111111e-1_f64 * t59670 - 0.11415555555555555555e-1_f64 * t59674 - 0.2283111111111111111e-1_f64 * t59678 + 0.11415555555555555555e-1_f64 * t59680 - 0.17123333333333333333e-1_f64 * t59684;
    t59802
}
