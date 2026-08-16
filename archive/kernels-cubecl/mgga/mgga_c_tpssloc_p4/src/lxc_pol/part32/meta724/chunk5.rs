//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2323/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2323<F: Float>(t29643: F, t3503: F, t86264: F, t1210: F, t29647: F, t8040: F, t95332: F, t29561: F, t6739: F, t7325: F, t1215: F, t15394: F, t18206: F, t18211: F, t18232: F, t18573: F, t2121: F, t2140: F, t24821: F, t27636: F, t27642: F, t27697: F, t488: F, t4899: F, t5011: F, t6224: F, t7331: F, t7999: F, t85972: F, t95396: F, t95446: F, t95450: F) -> F {
    let t104181 = t86264 * t3503 * t29643;
    let t104184 = t86264 * t1210 * t29647;
    let t104187 = t95332 * t8040;
    let t104190 = t29561 * t6739 * t7325;
    let t104193 = t18573 * t2140 * t488 / F::cast_from(1536.0_f64) - F::cast_from(0.60559134141210586284e-3_f64) * t95396 * t3503 * t6224 * t85972 * t1215 - F::cast_from(0.20186378047070195428e-3_f64) * t27636 * t27642 * t24821 * t5011 - F::cast_from(2.0_f64) / F::cast_from(81.0_f64) * t7999 * t27697 + t2121 * t4899 * t18232 / F::cast_from(216.0_f64) + t2121 * t4899 * t18211 / F::cast_from(36.0_f64) - F::cast_from(7.0_f64) / F::cast_from(648.0_f64) * t2121 * t15394 * t18206 + F::cast_from(0.20186378047070195428e-3_f64) * t104181 - F::cast_from(0.10093189023535097714e-3_f64) * t104184 + t95446 + t95450 / F::cast_from(81.0_f64) - F::cast_from(0.20186378047070195428e-3_f64) * t104187 + F::cast_from(0.72670960969452703541e-2_f64) * t104190 * t7331;
    t104193
}
