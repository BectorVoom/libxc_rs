//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 832/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk832<F: Float>(t3772: F, t7731: F, t169: F, t7766: F, t844: F, t2149: F, t849: F, t873: F, t164: F, t2210: F, t3750: F, t3753: F, t4034: F, t4040: F, t4042: F, t4065: F, t4067: F, t4070: F, t4077: F, t7598: F, t7602: F) -> F {
    let t8453 = t3772 * t7731;
    let t8466 = t844 * t169 * t7766;
    let t8470 = t849 * t873 * t2149;
    let t8473 = -F::cast_from(0.04115066352984959_f64) * t4034 - F::cast_from(0.04115066352984959_f64) * t4040 + F::cast_from(4.937333717448355_f64) * t4042 - F::cast_from(2.427516195194328_f64) * t8453 - F::cast_from(4.855032390388656_f64) * t3750 * t7598 - F::cast_from(2.427516195194328_f64) * t3750 * t7602 + F::cast_from(2.427516195194328_f64) * t3753 * t2210 - F::cast_from(2.2140749178833072_f64) * t4065 - F::cast_from(2.2140749178833072_f64) * t4067 + F::cast_from(2.2140749178833072_f64) * t4070 + F::cast_from(1.9882715304939877_f64) * t4077 + F::cast_from(0.04115066352984959_f64) * t164 * t8466 + F::cast_from(0.04115066352984959_f64) * t164 * t8470;
    t8473
}
