//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1151/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1151<F: Float>(t116: F, t786: F, t9534: F, t39568: F, t761: F, t39382: F, t39302: F, t6589: F, t68: F, t236: F, t40931: F, t240: F, t812: F) -> (F, F, F, F, F, F) {
    let t41214 = t9534 * t786 * t116;
    let t41254 = F::cast_from(0.14035736694323150897e2_f64) * t761 * t39568;
    let t41258 = F::cast_from(0.91082604192152556044e5_f64) * t761 * t39382;
    let t41262 = F::cast_from(0.5848223622634646207e0_f64) * t761 * t39302;
    let t41315 = t68 * t6589;
    let t41347 = t40931 * t236;
    let t41349 = t812 * t41347 * t240;
    (t41214, t41254, t41258, t41262, t41315, t41349)
}
