//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2323/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2323(t29643: f64, t3503: f64, t86264: f64, t1210: f64, t29647: f64, t8040: f64, t95332: f64, t29561: f64, t6739: f64, t7325: f64, t1215: f64, t15394: f64, t18206: f64, t18211: f64, t18232: f64, t18573: f64, t2121: f64, t2140: f64, t24821: f64, t27636: f64, t27642: f64, t27697: f64, t488: f64, t4899: f64, t5011: f64, t6224: f64, t7331: f64, t7999: f64, t85972: f64, t95396: f64, t95446: f64, t95450: f64) -> f64 {
    let t104181 = t86264 * t3503 * t29643;
    let t104184 = t86264 * t1210 * t29647;
    let t104187 = t95332 * t8040;
    let t104190 = t29561 * t6739 * t7325;
    let t104193 = t18573 * t2140 * t488 / 1536.0_f64 - 0.60559134141210586284e-3_f64 * t95396 * t3503 * t6224 * t85972 * t1215 - 0.20186378047070195428e-3_f64 * t27636 * t27642 * t24821 * t5011 - 2.0_f64 / 81.0_f64 * t7999 * t27697 + t2121 * t4899 * t18232 / 216.0_f64 + t2121 * t4899 * t18211 / 36.0_f64 - 7.0_f64 / 648.0_f64 * t2121 * t15394 * t18206 + 0.20186378047070195428e-3_f64 * t104181 - 0.10093189023535097714e-3_f64 * t104184 + t95446 + t95450 / 81.0_f64 - 0.20186378047070195428e-3_f64 * t104187 + 0.72670960969452703541e-2_f64 * t104190 * t7331;
    t104193
}
