//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 715/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk715<F: Float>(t2288: F, t650: F, t825: F, t96: F, t1011: F, t143: F, t155: F, t164: F, t205: F, t2198: F, t2202: F, t2419: F, t713: F, t7706: F, t7776: F, t7784: F, t7786: F, t7790: F, t7792: F, t7948: F, t7952: F, t7956: F, t7962: F, t933: F, t98: F) -> (F,) {
    let t7967 = t96 * t650 * t2288 * t825;
    let t7970 = 0.6268457032291772 * t933 * t2198 + 3.7610742193750633 * t143 * t7776 + 3.7610742193750633 * t143 * t7706 + 0.6268457032291772 * t933 * t2202 + 2.427516195194328 * t7784 * t7786 + 2.427516195194328 * t7790 + 2.427516195194328 * t7792 * t713 - 2.427516195194328 * t7948 * t98 - 2.3693919160612835 * t205 * t7952 - 2.3693919160612835 * t205 * t7956 + 1.8805371096875316 * t2419 * t1011 - 19.489173774580152 * t155 * t7962 - 0.04115066352984959 * t164 * t7967;
    (t7970,)
}
