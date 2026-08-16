//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 486/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk486(t1981: f64, t1983: f64, t1910: f64, t1914: f64, t1918: f64, t1922: f64, t1927: f64, t1930: f64, t1932: f64, t1935: f64, t1937: f64, t1938: f64, t1939: f64, t1959: f64, t1965: f64, t1971: f64, t1974: f64, t1976: f64, t1979: f64, t224: f64) -> (f64, f64) {
    let t1985 = 2.0_f64 / 45.0_f64 * t1981 * t1983;
    let t1986 = -t1910 - t1914 - t1918 + t1922 - t1927 + t1930 + t1932 + t1935 + t1937 - t1938 - 2.0_f64 / 45.0_f64 * t1939 - t1959 * t224 / 15.0_f64 + t1965 + t1971 + t1974 + t1976 + t1979 - t1985;
    (t1985, t1986)
}
