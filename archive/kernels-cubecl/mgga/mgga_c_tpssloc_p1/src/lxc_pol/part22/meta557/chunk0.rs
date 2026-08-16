//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2059/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2059<F: Float>(t116: F, t786: F, t9534: F, t133: F, t6600: F, t776: F, t39568: F, t761: F, t39382: F, t2531: F, t9713: F, t39302: F) -> (F, F, F, F, F, F) {
    let t41214 = t9534 * t786 * t116;
    let t41217 = t41214 * t133 * t6600 * t776;
    let t41254 = F::cast_from(0.14035736694323150897e2_f64) * t761 * t39568;
    let t41258 = F::cast_from(0.91082604192152556044e5_f64) * t761 * t39382;
    let t41259 = t2531 * t9713;
    let t41262 = F::cast_from(0.5848223622634646207e0_f64) * t761 * t39302;
    (t41214, t41217, t41254, t41258, t41259, t41262)
}
