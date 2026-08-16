//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1109/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1109(t7433: f64, t9633: f64, t30371: f64, t5940: f64, t7575: f64, t8480: f64, t8609: f64, t1165: f64, t6079: f64, t7351: f64, t7426: f64, t34501: f64, t34507: f64, t34513: f64, t34516: f64, t34526: f64, t34535: f64, t34538: f64, t37130: f64, t37131: f64, t39356: f64, t39358: f64, t39362: f64, t39364: f64) -> f64 {
    let t39366 = t7433 * t9633;
    let t39368 = t30371 * t5940;
    let t39373 = t7575 * t8480 * t8609;
    let t39377 = t7426 * t1165 * t7351 * t6079;
    let t39379 = -0.10482697429868050689e-2_f64 * t39356 - 0.94344276868812456204e-3_f64 * t39358 + 0.21437009059034868486e-3_f64 * t39362 + 0.17149607247227894789e-2_f64 * t39364 + t34501 + 0.12862205435420921092e-2_f64 * t39366 + t34507 - t37130 + t37131 - 0.34299214494455789578e-2_f64 * t39368 - t34513 + 0.20965394859736101378e-2_f64 * t34516 + 0.83861579438944405514e-3_f64 * t34526 + 0.21437009059034868486e-2_f64 * t39373 - 0.47172138434406228102e-3_f64 * t39377 - t34535 + t34538;
    t39379
}
