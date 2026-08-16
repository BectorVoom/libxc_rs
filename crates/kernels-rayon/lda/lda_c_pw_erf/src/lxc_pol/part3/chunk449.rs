//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 449/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk449(t138: f64, t1704: f64, t1706: f64, t1711: f64, t1712: f64, t1724: f64, t444: f64, t450: f64, t101: f64, t100: f64, t95: f64) -> (f64, f64, f64) {
    let t1726 = t1704 * t138 - 2.0_f64 * t1706 * t450 + 2.0_f64 * t1711 * t1712 - t444 * t1724;
    let t1727 = t101 * t1726;
    let t1729 = t95 * t100;
    (t1726, t1727, t1729)
}
