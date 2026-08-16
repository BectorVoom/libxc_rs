//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1109/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1109<F: Float>(t7433: F, t9633: F, t30371: F, t5940: F, t7575: F, t8480: F, t8609: F, t1165: F, t6079: F, t7351: F, t7426: F, t34501: F, t34507: F, t34513: F, t34516: F, t34526: F, t34535: F, t34538: F, t37130: F, t37131: F, t39356: F, t39358: F, t39362: F, t39364: F) -> F {
    let t39366 = t7433 * t9633;
    let t39368 = t30371 * t5940;
    let t39373 = t7575 * t8480 * t8609;
    let t39377 = t7426 * t1165 * t7351 * t6079;
    let t39379 = -F::cast_from(0.10482697429868050689e-2_f64) * t39356 - F::cast_from(0.94344276868812456204e-3_f64) * t39358 + F::cast_from(0.21437009059034868486e-3_f64) * t39362 + F::cast_from(0.17149607247227894789e-2_f64) * t39364 + t34501 + F::cast_from(0.12862205435420921092e-2_f64) * t39366 + t34507 - t37130 + t37131 - F::cast_from(0.34299214494455789578e-2_f64) * t39368 - t34513 + F::cast_from(0.20965394859736101378e-2_f64) * t34516 + F::cast_from(0.83861579438944405514e-3_f64) * t34526 + F::cast_from(0.21437009059034868486e-2_f64) * t39373 - F::cast_from(0.47172138434406228102e-3_f64) * t39377 - t34535 + t34538;
    t39379
}
