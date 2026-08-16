//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 316/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk316(t1810: f64, t277: f64, t128: f64, t1704: f64, t793: f64, t1773: f64, t797: f64, t1776: f64, t305: f64, t1734: f64, t1737: f64, t838: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1923 = t277 * t1810;
    let t1926 = t128 * t1704;
    let t1927 = t793 * t1926;
    let t1929 = t797 * t1773;
    let t1931 = t305 * t1776;
    let t1933 = t128 * t1734;
    let t1934 = t305 * t1933;
    let t1936 = t128 * t1737;
    let t1937 = t838 * t1936;
    (t1923, t1926, t1927, t1929, t1931, t1933, t1934, t1936, t1937)
}
