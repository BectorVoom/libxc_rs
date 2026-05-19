//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1360/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1360<F: Float>(t1385: F, t1943: F, t27370: F, t98144: F, t102051: F, t102054: F, t102057: F, t102072: F, t103149: F, t103199: F, t12185: F, t1307: F, t27369: F, t28443: F, t29289: F, t3984: F, t7908: F, t94208: F, t94287: F, t98119: F, t98226: F) -> (F, F, F) {
    let t103301 = t1943 * t1385;
    let t103303 = t27370 * t98144 * t103301;
    let t103318 = -F::cast_from(0.11054629629629629629e-1_f64) * t102051 - F::cast_from(0.46336805555555555557e-3_f64) * t7908 * t12185 * t103199 * t1307 + F::cast_from(0.55652820312500000001e-3_f64) * t27369 * t103303 - F::cast_from(0.58958024691358024689e-2_f64) * t102054 + F::cast_from(0.22109259259259259259e-2_f64) * t102057 + F::cast_from(0.23168402777777777778e-3_f64) * t7908 * t3984 * t103149 * t1307 + F::cast_from(0.6183646701388888889e-4_f64) * t98119 * t28443 + F::cast_from(0.88437037037037037033e-2_f64) * t102072 + t98226 + F::cast_from(0.51485339506172839507e-4_f64) * t94287 - F::cast_from(0.18550940104166666667e-3_f64) * t94208 * t29289;
    (t103301, t103303, t103318)
}
