//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2304/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2304(t3302: f64, t357: f64, t4982: f64, t999: f64, t1647: f64, t4980: f64, t4995: f64, t1678: f64, t3298: f64, t342: f64, t3316: f64, t1045: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t19482 = t3302 * t357;
    let t19502 = t4982 * t999;
    let t19526 = t1647 * t4980;
    let t19569 = t1647 * t4995;
    let t19579 = t19482 * t999;
    let t19602 = t3298 * t1678;
    let t19603 = t342 * t19602;
    let t19607 = t3316 * t1678;
    let t19608 = t342 * t19607;
    let t19620 = t1045 * t999;
    (t19502, t19526, t19569, t19579, t19602, t19603, t19607, t19608, t19620)
}
