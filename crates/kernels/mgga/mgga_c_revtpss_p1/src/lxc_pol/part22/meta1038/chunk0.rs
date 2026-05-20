//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3628/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3628<F: Float>(t20473: F, t3531: F, t16685: F, t5192: F, t16652: F, t57854: F, t1196: F, t12500: F, t20472: F, t20892: F, t20384: F, t3497: F, t45187: F, t45190: F, t6518: F) -> (F, F, F, F, F, F, F) {
    let t68707 = F::cast_from(0.20779030926817756511e3_f64) * t3531 * t20473;
    let t68709 = F::cast_from(0.34631718211362927517e2_f64) * t5192 * t16685;
    let t68711 = F::new(24.0) * t57854 * t16652;
    let t68714 = F::cast_from(0.10389515463408878255e3_f64) * t1196 * t20472 * t12500;
    let t68716 = F::cast_from(0.20508037716432813316e4_f64) * t3531 * t20892;
    let t68718 = F::cast_from(0.11696447245269292414e1_f64) * t3531 * t20384;
    let t68723 = F::cast_from(0.91082604192152556044e5_f64) * t1196 * t45187 * t6518 * t45190 * t3497;
    (t68707, t68709, t68711, t68714, t68716, t68718, t68723)
}
