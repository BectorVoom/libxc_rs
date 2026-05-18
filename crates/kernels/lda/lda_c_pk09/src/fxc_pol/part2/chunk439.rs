//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 439/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk439<F: Float>(t2288: F, t61: F, t825: F, t96: F, t151: F, t155: F, t161: F, t164: F, t192: F, t2154: F, t2210: F, t2214: F, t2239: F, t2248: F, t2252: F, t2256: F, t2260: F, t2264: F, t2271: F, t2275: F, t2279: F, t98: F) -> (F, F) {
    let t2291 = t96 * t61 * t2288 * t825;
    let t2300 = F::new(2.2140749178833072) * t192 * t2210 + F::new(2.2140749178833072) * t192 * t2214 - F::new(2.2140749178833072) * t2239 * t98 - F::new(4.937333717448355) * t161 * t2210 - F::new(4.937333717448355) * t161 * t2214 + F::new(22.07984838129906) * t2248 + F::new(22.07984838129906) * t2252 + F::new(1.800081713982063) * t2256 + F::new(1.800081713982063) * t2260 - F::new(1.800081713982063) * t2264 + F::new(4.937333717448355) * t2271 * t98 + F::new(0.04115066352984959) * t164 * t2275 + F::new(0.04115066352984959) * t164 * t2279 - F::new(0.04115066352984959) * t164 * t2291 - F::new(1.8805371096875316) * t151 * t2210 + F::new(1.8805371096875316) * t151 * t2154 + F::new(19.489173774580152) * t155 * t2154;
    (t2291, t2300)
}
