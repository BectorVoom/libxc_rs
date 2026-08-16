//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1074/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1074<F: Float>(t11679: F, t471: F, t2042: F, t10954: F, t10966: F, t11062: F, t11070: F, t11464: F, t11467: F, t11470: F, t11673: F, t2032: F, t2813: F, t2826: F, t6288: F, t7241: F, t7244: F, t7253: F, t7256: F, t7276: F, t7279: F, t7297: F, t7302: F, t7310: F) -> F {
    let t11680 = t471 * t11679;
    let t11681 = t11680 * t2042;
    let t11688 = F::cast_from(0.10237773105191754_f64) * t11070 + F::cast_from(0.10237773105191754_f64) * t10954 + F::cast_from(0.10237773105191754_f64) * t10966 + F::cast_from(0.10237773105191754_f64) * t11062 + F::cast_from(0.04991874779241519_f64) * t11464 + F::cast_from(0.02466859483068398_f64) * t11467 - F::cast_from(0.02466859483068398_f64) * t11470 + t7241 / F::cast_from(6.0_f64) + t7244 / F::cast_from(6.0_f64) + t7253 / F::cast_from(6.0_f64) - t7256 / F::cast_from(6.0_f64) - t11673 / F::cast_from(6.0_f64) - t7276 / F::cast_from(12.0_f64) - t7279 / F::cast_from(6.0_f64) - t2826 * t2032 / F::cast_from(6.0_f64) - t11681 / F::cast_from(6.0_f64) - t2813 * t6288 / F::cast_from(6.0_f64) - t7297 / F::cast_from(6.0_f64) + t7302 / F::cast_from(6.0_f64) + t7310 / F::cast_from(6.0_f64);
    t11688
}
