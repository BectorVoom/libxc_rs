//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 944/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk944<F: Float>(t11679: F, t471: F, t2042: F, t10954: F, t10966: F, t11062: F, t11070: F, t11464: F, t11467: F, t11470: F, t11673: F, t2032: F, t2813: F, t2826: F, t6288: F, t7241: F, t7244: F, t7253: F, t7256: F, t7276: F, t7279: F, t7297: F, t7302: F, t7310: F) -> (F,) {
    let t11680 = t471 * t11679;
    let t11681 = t11680 * t2042;
    let t11688 = 0.10237773105191754 * t11070 + 0.10237773105191754 * t10954 + 0.10237773105191754 * t10966 + 0.10237773105191754 * t11062 + 0.04991874779241519 * t11464 + 0.02466859483068398 * t11467 - 0.02466859483068398 * t11470 + t7241 / 6.0 + t7244 / 6.0 + t7253 / 6.0 - t7256 / 6.0 - t11673 / 6.0 - t7276 / 12.0 - t7279 / 6.0 - t2826 * t2032 / 6.0 - t11681 / 6.0 - t2813 * t6288 / 6.0 - t7297 / 6.0 + t7302 / 6.0 + t7310 / 6.0;
    (t11688,)
}
