//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 455/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk455<F: Float>(t44: F, t2455: F, t275: F, t1191: F, t2140: F, t271: F, t1197: F, t1193: F, t1195: F, t276: F, t2146: F, t1207: F, t1204: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F) {
    let t45 = t44 <= zeta_threshold;
    let t2456 = t275 * t2455;
    let t2457 = t2456 * t1191;
    let t2459 = t271 * t2140;
    let t2460 = t2459 * t1197;
    let t2463 = t2455 * t1193 + F::cast_from(1.28_f64) * t1195 * t2460;
    let t2464 = t276 * t2463;
    let t2465 = piecewise3::<F>(t45, t2457, t2464);
    let t2467 = t271 * t2146;
    let t2468 = t2467 * t1207;
    let t2471 = t2455 * t1204 + F::cast_from(1.28_f64) * t1195 * t2468;
    (t2457, t2459, t2460, t2463, t2465, t2467, t2468, t2471)
}
