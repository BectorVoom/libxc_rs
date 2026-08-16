//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2864/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2864(t41658: f64, t41675: f64, t41684: f64, t59655: f64, t59657: f64, t59661: f64, t59663: f64, t59665: f64, t59670: f64, t59674: f64, t59678: f64, t59680: f64, t59684: f64) -> f64 {
    let t59860 = -0.52765432098765432098e-2_f64 * t41658 + 0.15829629629629629629e-1_f64 * t41675 + 0.36935802469135802468e-1_f64 * t41684 - 0.4274e0_f64 * t59655 - 0.52765432098765432097e-2_f64 * t59657 + 0.4274e0_f64 * t59661 - 0.23744444444444444444e-1_f64 * t59663 + 0.79148148148148148146e-2_f64 * t59665 - 0.23744444444444444444e-1_f64 * t59670 - 0.11872222222222222222e-1_f64 * t59674 - 0.23744444444444444444e-1_f64 * t59678 + 0.11872222222222222222e-1_f64 * t59680 - 0.17808333333333333333e-1_f64 * t59684;
    t59860
}
