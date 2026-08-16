//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2731/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2731(t20900: f64, t73: f64, t12987: f64, t5390: f64, t12772: f64, t17736: f64, t21309: f64, t3767: f64, t70629: f64, t474: f64, t6593: f64, t3089: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t70944 = t20900 * t73;
    let t70959 = t12987 * t5390;
    let t70982 = t17736 * t12772 * t21309;
    let t70990 = t3767 * t70629;
    let t70993 = t474 * t6593;
    let t70994 = t70993 * t3089;
    (t70944, t70959, t70982, t70990, t70993, t70994)
}
