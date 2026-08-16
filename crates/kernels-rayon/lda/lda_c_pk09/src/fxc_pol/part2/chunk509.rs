//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 509/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk509(t2730: f64, t545: f64, t1842: f64, t1847: f64, t1856: f64, t1954: f64, t1956: f64, t1958: f64, t1994: f64, t2002: f64, t2019: f64, t2023: f64, t2025: f64, t2744: f64, t2748: f64, t2752: f64, t2940: f64, t2943: f64, t455: f64) -> (f64, f64) {
    let t2946 = t545 * t2730;
    let t2949 = -7.108175748183851_f64 * t1842 * t2744 + 7.108175748183851_f64 * t1847 * t2748 + 2.427516195194328_f64 * t1856 * t2752 - 2.427516195194328_f64 * t2940 * t455 - t1954 - t1956 + t1958 + t1994 + t2002 + t2019 + t2023 + t2025 - 2.2140749178833072_f64 * t2943 * t455 - 2.427516195194328_f64 * t2946 * t455;
    (t2946, t2949)
}
