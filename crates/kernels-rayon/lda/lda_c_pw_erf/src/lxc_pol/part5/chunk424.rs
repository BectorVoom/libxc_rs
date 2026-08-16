//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 424/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk424(t1982: f64, t199: f64, t568: f64, t822: f64, t1326: f64, t1972: f64, t519: f64, t1608: f64, t1611: f64, t1615: f64, t1623: f64, t1922: f64, t1923: f64, t1927: f64, t1929: f64, t1931: f64, t1934: f64, t1936: f64, t1937: f64, t1939: f64, t1962: f64, t231: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1984 = 2.0_f64 / 15.0_f64 * t1982 * t199;
    let t1985 = t822 * t568;
    let t1986 = 4.0_f64 / 45.0_f64 * t1985;
    let t1987 = t1326 * t1972;
    let t1989 = 8.0_f64 / 45.0_f64 * t519 * t1987;
    let t1990 = 4.0_f64 / 3.0_f64 * t1608 + t1611 - t1922 - t1923 + 4.0_f64 / 3.0_f64 * t1615 + t1623 + 0.10821041362364843_f64 * t1927 + 4.0_f64 / 3.0_f64 * t1929 + 4.0_f64 / 3.0_f64 * t1931 * t231 + 4.0_f64 / 3.0_f64 * t1934 + t1936 + t1937 + t1939 + t1962 + t1984 + t1986 - t1989;
    (t1984, t1985, t1986, t1987, t1989, t1990)
}
