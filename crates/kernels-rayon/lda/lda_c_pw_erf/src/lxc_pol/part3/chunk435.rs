//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 435/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk435(t1624: f64, t1625: f64, t1642: f64, t1643: f64, t1203: f64, t1573: f64, t1574: f64, t1577: f64, t1581: f64, t1584: f64, t1588: f64, t1592: f64, t1595: f64, t1599: f64, t1603: f64, t1606: f64, t163: f64, t164: f64, t169: f64, t171: f64) -> (f64, f64) {
    let t1645 = t1624 + t1625 + t1642 + t1643;
    let t1650 = -t1573 + 0.06301081444628223_f64 * t1574 + t1577 + t1581 - 0.031505407223141116_f64 * t1203 * t164 - 0.06301081444628223_f64 * t1584 - 0.003950778065781896_f64 * t1588 - t1592 - t1595 - t1599 - t1603 + 0.017961351015381915_f64 * t1606 - 0.005388405304614574_f64 * t169 * t171 * t1645 * t163;
    (t1645, t1650)
}
