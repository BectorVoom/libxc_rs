//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 753/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk753(t14935: f64, t874: f64, t70188: f64, t70271: f64, t70316: f64, t69287: f64, t3281: f64, t4616: f64, t70610: f64, t13964: f64, t14065: f64, t14092: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t73450 = t874 * t14935;
    let t73454 = 0.46328831667894726561e-5_f64 * t70188;
    let t73480 = 0.65053455985619242964e-5_f64 * t70271;
    let t73484 = 0.65053455985619242964e-5_f64 * t70316;
    let t73536 = 0.30643330512125015891e-2_f64 * t69287;
    let t73569 = t4616 * t3281;
    let t73624 = 0.65053455985619242964e-5_f64 * t70610;
    let t73645 = 0.13010691197123848593e-4_f64 * t13964;
    let t73658 = 0.58171619854173713844e-4_f64 * t14065;
    let t73659 = 0.114000641766744825e-6_f64 * t14092;
    (t73450, t73454, t73480, t73484, t73536, t73569, t73624, t73645, t73658, t73659)
}
