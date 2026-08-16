//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 532/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk532(t2657: f64, t2663: f64, t2665: f64, t2666: f64, t1573: f64, t1577: f64, t1581: f64, t1592: f64, t1595: f64, t1599: f64, t1603: f64, t163: f64, t164: f64, t169: f64, t171: f64, t1902: f64, t1908: f64, t1912: f64, t1919: f64, t2379: f64) -> (f64, f64) {
    let t2668 = t2657 + t2663 + t2665 + t2666;
    let t2673 = -t1573 + 0.06301081444628223_f64 * t1902 + t1577 + t1581 - 0.031505407223141116_f64 * t2379 * t164 - 0.06301081444628223_f64 * t1908 - 0.003950778065781896_f64 * t1912 - t1592 - t1595 - t1599 - t1603 + 0.017961351015381915_f64 * t1919 - 0.005388405304614574_f64 * t169 * t171 * t2668 * t163;
    (t2668, t2673)
}
