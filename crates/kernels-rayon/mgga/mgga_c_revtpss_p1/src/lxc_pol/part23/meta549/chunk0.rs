//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2102/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2102(t1390: f64, t22253: f64, t828: f64, t221: f64, t4019: f64, t6844: f64, t4018: f64, t14045: f64, t6869: f64, t3992: f64, t2661: f64, t6874: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22255 = t1390 * t828 * t22253;
    let t22259 = t4019 * t221 * t6844;
    let t22260 = t4018 * t22259;
    let t22262 = t14045 * t6869;
    let t22263 = t3992 * t22262;
    let t22264 = t2661 * t22263;
    let t22267 = t4019 * t221 * t6874;
    (t22255, t22259, t22260, t22263, t22264, t22267)
}
