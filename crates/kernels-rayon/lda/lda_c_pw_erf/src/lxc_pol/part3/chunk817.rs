//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 817/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk817(t5658: f64, t5666: f64, t138: f64, t1706: f64, t1711: f64, t1712: f64, t1724: f64, t1861: f64, t1864: f64, t1878: f64, t3329: f64, t3332: f64, t3339: f64, t444: f64, t450: f64, t5616: f64, t5618: f64, t5621: f64, t5630: f64, t5633: f64, t5636: f64, t774: f64) -> (f64, f64) {
    let t5667 = t5658 + t5666;
    let t5669 = t5616 * t138 - 2.0_f64 * t1706 * t1878 + 4.0_f64 * t1711 * t5633 + 2.0_f64 * t1711 * t5636 + 2.0_f64 * t5621 * t1712 - t1861 * t1724 + 4.0_f64 * t3332 * t1864 - t3329 * t774 - 6.0_f64 * t3339 * t5630 - t444 * t5667 - 2.0_f64 * t5618 * t450;
    (t5667, t5669)
}
