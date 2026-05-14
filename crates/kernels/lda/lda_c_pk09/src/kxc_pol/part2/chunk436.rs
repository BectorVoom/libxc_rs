//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 436/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk436<F: Float>(t2520: F, t93: F, t339: F, t1221: F, t2475: F, t2478: F, t2481: F, t2484: F, t2488: F, t2491: F, t2494: F, t2497: F, t2509: F, t2514: F, t2518: F, t311: F, t374: F, t410: F) -> (F, F, F) {
    let t2521 = t93 * t2520;
    let t2522 = t339 * t2521;
    let t2524 = -2.2140749178833072 * t2475 * t311 + 18.635258017632964 * t2478 * t311 - 2.427516195194328 * t2481 * t311 + 4.937333717448355 * t2484 * t311 - 0.04115066352984959 * t2488 * t374 + 19.489173774580152 * t2491 * t311 + 1.8805371096875316 * t2494 * t311 - 3.7610742193750633 * t2497 * t311 - 1.8805371096875316 * t2509 * t410 + 22.07984838129906 * t2514 - 5.40024514194619 * t2518 + 5.40024514194619 * t2522 + t1221;
    (t2521, t2522, t2524)
}
