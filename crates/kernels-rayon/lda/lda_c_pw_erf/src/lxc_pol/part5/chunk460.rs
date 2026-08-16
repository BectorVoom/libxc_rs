//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 460/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk460(t1990: f64, t2042: f64, t2126: f64, t2196: f64, t1573: f64, t1574: f64, t1577: f64, t1581: f64, t1584: f64, t1588: f64, t1592: f64, t1595: f64, t1599: f64, t1603: f64, t1606: f64, t163: f64, t164: f64, t169: f64, t171: f64, t1902: f64, t1905: f64, t1908: f64, t1912: f64, t1919: f64) -> (f64, f64) {
    let t2198 = t1990 + t2042 + t2126 + t2196;
    let t2203 = -t1573 + 0.031505407223141116_f64 * t1574 + t1577 + t1581 + 0.031505407223141116_f64 * t1902 - 0.031505407223141116_f64 * t1905 * t164 - 0.031505407223141116_f64 * t1908 - 0.001975389032890948_f64 * t1912 - 0.031505407223141116_f64 * t1584 - t1592 - t1595 - 0.001975389032890948_f64 * t1588 - t1599 - t1603 + 0.008980675507690957_f64 * t1606 + 0.008980675507690957_f64 * t1919 - 0.005388405304614574_f64 * t169 * t171 * t2198 * t163;
    (t2198, t2203)
}
