//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 400/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk400<F: Float>(t1729: F, t501: F, t1805: F, t1929: F, t1933: F, t1934: F, t1944: F, t1952: F, t1954: F, t1956: F, t1958: F, t1959: F, t1962: F, t1965: F, t1968: F, t1972: F, t455: F, t516: F) -> (F, F) {
    let t1975 = t501 * t1729;
    let t1978 = -t1929 - t1933 - F::cast_from(4.937333717448355_f64) * t1934 * t1805 + F::cast_from(4.937333717448355_f64) * t1944 * t455 + t1952 - t1954 - t1956 + t1958 - F::cast_from(2.427516195194328_f64) * t1959 * t455 - F::cast_from(2.2140749178833072_f64) * t1962 * t455 + F::cast_from(18.635258017632964_f64) * t1965 * t455 + F::cast_from(4.937333717448355_f64) * t1968 * t455 - F::cast_from(0.04115066352984959_f64) * t1972 * t516 + F::cast_from(19.489173774580152_f64) * t1975 * t455;
    (t1975, t1978)
}
