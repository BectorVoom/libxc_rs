//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 78/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk78<F: Float>(t122: F, t123: F, t134: F, t135: F, t137: F, t143: F, t151: F, t155: F, t161: F, t164: F, t170: F, t179: F, t192: F, t200: F, t205: F, t80: F) -> F {
    let t208 = t80 * t137 - F::new(22.07984838129906) * t123 - F::new(0.9000408569910315) * t135 - F::new(3.7610742193750633) * t143 * t122 + F::new(1.8805371096875316) * t151 * t122 + F::new(19.489173774580152) * t155 * t122 + F::new(4.937333717448355) * t161 * t122 - F::new(0.04115066352984959) * t164 * t170 + F::new(18.635258017632964) * t179 * t122 - F::new(2.2140749178833072) * t192 * t122 - F::new(2.427516195194328) * t200 * t122 - F::new(1.1846959580306418) * t205 * t134;
    t208
}
