//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 509/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk509<F: Float>(t2730: F, t545: F, t1842: F, t1847: F, t1856: F, t1954: F, t1956: F, t1958: F, t1994: F, t2002: F, t2019: F, t2023: F, t2025: F, t2744: F, t2748: F, t2752: F, t2940: F, t2943: F, t455: F) -> (F, F) {
    let t2946 = t545 * t2730;
    let t2949 = -F::cast_from(7.108175748183851_f64) * t1842 * t2744 + F::cast_from(7.108175748183851_f64) * t1847 * t2748 + F::cast_from(2.427516195194328_f64) * t1856 * t2752 - F::cast_from(2.427516195194328_f64) * t2940 * t455 - t1954 - t1956 + t1958 + t1994 + t2002 + t2019 + t2023 + t2025 - F::cast_from(2.2140749178833072_f64) * t2943 * t455 - F::cast_from(2.427516195194328_f64) * t2946 * t455;
    (t2946, t2949)
}
