//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 469/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk469(t1798: f64, t183: f64, t539: f64, t794: f64, t188: f64, t27: f64, t856: f64, t545: f64, t1404: f64, t1412: f64, t1918: f64, t1922: f64, t1927: f64, t1930: f64, t1932: f64, t1935: f64, t1937: f64, t1938: f64, t1965: f64, t1971: f64, t1974: f64, t1976: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2342 = t1798 * t183;
    let t2345 = t794 * t539;
    let t2346 = t2345 * t188;
    let t2349 = t856 * t27;
    let t2350 = t2349 * t545;
    let t2352 = -t1918 + t1922 - t1927 + t1930 + t1932 + t1935 + t1937 - t1938 + 4.0_f64 / 3.0_f64 * t2342 * t188 + 4.0_f64 / 3.0_f64 * t2346 + 4.0_f64 / 3.0_f64 * t1404 + t1412 + 0.10821041362364843_f64 * t2350 + t1965 + t1971 + t1974 + t1976;
    (t2342, t2345, t2346, t2349, t2350, t2352)
}
