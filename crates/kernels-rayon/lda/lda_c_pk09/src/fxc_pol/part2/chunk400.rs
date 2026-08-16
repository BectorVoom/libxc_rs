//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 400/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk400(t1729: f64, t501: f64, t1805: f64, t1929: f64, t1933: f64, t1934: f64, t1944: f64, t1952: f64, t1954: f64, t1956: f64, t1958: f64, t1959: f64, t1962: f64, t1965: f64, t1968: f64, t1972: f64, t455: f64, t516: f64) -> (f64, f64) {
    let t1975 = t501 * t1729;
    let t1978 = -t1929 - t1933 - 4.937333717448355_f64 * t1934 * t1805 + 4.937333717448355_f64 * t1944 * t455 + t1952 - t1954 - t1956 + t1958 - 2.427516195194328_f64 * t1959 * t455 - 2.2140749178833072_f64 * t1962 * t455 + 18.635258017632964_f64 * t1965 * t455 + 4.937333717448355_f64 * t1968 * t455 - 0.04115066352984959_f64 * t1972 * t516 + 19.489173774580152_f64 * t1975 * t455;
    (t1975, t1978)
}
