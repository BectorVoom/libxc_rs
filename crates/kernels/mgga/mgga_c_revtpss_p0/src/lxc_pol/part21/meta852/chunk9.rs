//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3211/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3211<F: Float>(t17795: F, t3172: F, t3711: F, t1214: F, t3604: F, t17729: F, t17759: F, t44425: F, t29048: F, t3362: F, t10326: F, t10356: F, t1042: F, t12787: F, t12920: F, t12931: F, t16719: F, t16724: F, t17344: F, t17429: F, t17482: F, t17500: F, t17558: F, t17580: F, t17633: F, t17687: F, t17688: F, t17730: F, t17781: F, t247: F, t3368: F, t3568: F, t3625: F, t3626: F, t3628: F, t3647: F, t3719: F, t3720: F, t4186: F, t44484: F, t44551: F, t45346: F, t45352: F, t5046: F, t51959: F, t5296: F, t5351: F, t5384: F, t56620: F, t57548: F, t58969: F) -> F {
    let t59269 = t3711 * t3172 * t17795;
    let t59279 = t3604 * t1214;
    let t59320 = t17729 * t44425 * t17759;
    let t59330 = t29048 * t3362;
    let t59334 = -F::cast_from(0.47637797908966374413e-3_f64) * t59269 + F::cast_from(0.71456696863449561621e-3_f64) * t3647 * t17558 + F::cast_from(0.14291339372689912324e-3_f64) * t45346 + F::cast_from(0.14291339372689912324e-3_f64) * t45352 - F::cast_from(0.12862205435420921092e-2_f64) * t44484 * t17580 - F::cast_from(0.12862205435420921092e-2_f64) * t17429 * t17781 + F::cast_from(0.25724410870841842183e-2_f64) * t44551 * t3720 * t17482 * t59279 + F::cast_from(0.7145669686344956162e-3_f64) * t3625 * t12787 * t17633 * t17688 - F::cast_from(0.14291339372689912324e-3_f64) * t3625 * t3626 * t5351 * t3628 * t10326 - F::cast_from(0.85748036236139473944e-3_f64) * t3625 * t3626 * t5351 * t17687 * t10356 - F::cast_from(0.85748036236139473944e-3_f64) * t5384 * t1042 * t5296 * t4186 * t3568 - F::cast_from(0.38586616306262763275e-2_f64) * t17344 * t247 * t3719 * t56620 - F::cast_from(0.14291339372689912324e-2_f64) * t17729 * t12787 * t16724 * t17730 - F::cast_from(0.71456696863449561621e-3_f64) * t17729 * t12787 * t5046 * t12931 - F::cast_from(0.71456696863449561621e-3_f64) * t17729 * t12787 * t5046 * t58969 - F::cast_from(0.95275595817932748827e-3_f64) * t59320 + F::cast_from(0.85748036236139473944e-3_f64) * t3711 * t1042 * t17500 * t3368 - F::cast_from(0.42874018118069736973e-2_f64) * t17729 * t12787 * t16719 * t12920 - t57548 * t59330 * t51959 / F::cast_from(16.0_f64);
    t59334
}
