//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2418/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2418<F: Float>(t207: F, t40344: F, t795: F, t116: F, t786: F, t9534: F, t133: F, t6600: F, t776: F, t39568: F, t761: F, t2535: F, t9716: F) -> (F, F, F, F, F) {
    let t41212 = F::cast_from(0.11265432098765432099e0_f64) * t40344 * t207 * t795;
    let t41214 = t9534 * t786 * t116;
    let t41217 = t41214 * t133 * t6600 * t776;
    let t41254 = F::cast_from(0.14035736694323150897e2_f64) * t761 * t39568;
    let t41255 = t9716 * t2535;
    (t41212, t41214, t41217, t41254, t41255)
}
