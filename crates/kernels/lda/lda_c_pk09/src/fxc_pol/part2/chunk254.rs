//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 254/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk254<F: Float>(t1075: F, t1120: F, t79: F, t137: F, t1091: F, t143: F, t155: F, t179: F, t192: F, t200: F, t205: F, t709: F, t713: F, t756: F, t80: F, t946: F, t949: F, t953: F, t959: F, t976: F, t98: F, t982: F, t986: F) -> (F, F, F, F, F) {
    let t1121 = t1075 + t1120;
    let t1123 = t79 * t79;
    let t1124 = F::new(1.0) / t1123;
    let t1125 = t1124 * t137;
    let t1127 = -F::new(2.2140749178833072) * t192 * t756 + F::new(18.635258017632964) * t179 * t756 - t946 + F::new(2.3693919160612835) * t205 * t949 - F::new(2.3693919160612835) * t205 * t953 + t959 + F::new(2.427516195194328) * t200 * t713 - F::new(19.489173774580152) * t155 * t713 - F::new(19.489173774580152) * t155 * t709 + F::new(19.489173774580152) * t976 * t98 - t982 + t986 + F::new(3.7610742193750633) * t143 * t713 + F::new(3.7610742193750633) * t143 * t709 + F::new(2.427516195194328) * t200 * t709 + t80 * t1121 - t1125 * t1091;
    (t1121, t1123, t1124, t1125, t1127)
}
