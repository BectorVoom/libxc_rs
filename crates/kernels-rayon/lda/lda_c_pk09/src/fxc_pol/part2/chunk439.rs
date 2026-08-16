//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 439/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk439(t2288: f64, t61: f64, t825: f64, t96: f64, t151: f64, t155: f64, t161: f64, t164: f64, t192: f64, t2154: f64, t2210: f64, t2214: f64, t2239: f64, t2248: f64, t2252: f64, t2256: f64, t2260: f64, t2264: f64, t2271: f64, t2275: f64, t2279: f64, t98: f64) -> (f64, f64) {
    let t2291 = t96 * t61 * t2288 * t825;
    let t2300 = 2.2140749178833072_f64 * t192 * t2210 + 2.2140749178833072_f64 * t192 * t2214 - 2.2140749178833072_f64 * t2239 * t98 - 4.937333717448355_f64 * t161 * t2210 - 4.937333717448355_f64 * t161 * t2214 + 22.07984838129906_f64 * t2248 + 22.07984838129906_f64 * t2252 + 1.800081713982063_f64 * t2256 + 1.800081713982063_f64 * t2260 - 1.800081713982063_f64 * t2264 + 4.937333717448355_f64 * t2271 * t98 + 0.04115066352984959_f64 * t164 * t2275 + 0.04115066352984959_f64 * t164 * t2279 - 0.04115066352984959_f64 * t164 * t2291 - 1.8805371096875316_f64 * t151 * t2210 + 1.8805371096875316_f64 * t151 * t2154 + 19.489173774580152_f64 * t155 * t2154;
    (t2291, t2300)
}
