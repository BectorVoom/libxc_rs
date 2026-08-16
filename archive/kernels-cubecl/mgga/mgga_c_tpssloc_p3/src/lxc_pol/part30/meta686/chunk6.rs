//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2173/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2173<F: Float>(t22892: F, t22893: F, t28138: F, t1336: F, t1352: F, t16060: F, t19810: F, t26404: F, t26442: F, t26456: F, t26458: F, t28152: F, t3777: F, t5234: F, t5287: F, t5344: F, t544: F, t553: F, t7745: F, t91065: F, t91077: F, t93795: F, t93796: F, t97172: F, t97181: F, t97189: F, t97200: F, t97468: F, t97488: F, t97491: F) -> F {
    let t97494 = t22892 * t22893 * t28138;
    let t97496 = -F::cast_from(2.0_f64) * t5344 * t97189 * t1352 - F::cast_from(0.19190897446562641759e-1_f64) * t97200 + t91065 + t544 * t553 * t97468 - F::cast_from(2.0_f64) * t1336 * t26458 * t5287 - F::cast_from(2.0_f64) * t16060 * t7745 - F::cast_from(2.0_f64) * t5234 * t26442 - F::cast_from(2.0_f64) * t5234 * t26456 - F::cast_from(2.0_f64) * t19810 * t26404 - t3777 * t28152 + t91077 - t93795 - t5344 * t97181 * t1352 + t93796 - t5344 * t97172 * t1352 + F::cast_from(0.16449340668482264365e-1_f64) * t97488 + F::cast_from(0.3289868133696452873e-1_f64) * t97491 + F::cast_from(0.82246703342411321825e-2_f64) * t97494;
    t97496
}
