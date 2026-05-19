//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 449/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk449<F: Float>(t1069: F, t1071: F, t1073: F, t1100: F, t1108: F, t1114: F, t1115: F, t1116: F, t1117: F, t2171: F, t2175: F, t2179: F, t2248: F, t2252: F, t2256: F, t2260: F, t2264: F, t2394: F, t98: F) -> F {
    let t2405 = -t2394 * t98 / F::new(6.0) - F::cast_from(0.10237773105191754_f64) * t2171 - F::cast_from(0.10237773105191754_f64) * t2175 + t1069 + t1071 - t1073 - t1100 + t1108 - F::cast_from(0.14975624337724558_f64) * t2248 - F::cast_from(0.14975624337724558_f64) * t2252 + F::cast_from(0.10237773105191754_f64) * t2179 - F::cast_from(0.01233429741534199_f64) * t2256 - F::cast_from(0.01233429741534199_f64) * t2260 + F::cast_from(0.01233429741534199_f64) * t2264 - t1114 - t1115 - t1116 - t1117;
    t2405
}
