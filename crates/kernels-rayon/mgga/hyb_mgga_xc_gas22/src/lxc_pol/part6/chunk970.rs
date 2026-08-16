//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 970/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk970(t2307: f64, t3435: f64, t1370: f64, t6640: f64, t2315: f64, t2292: f64, t3443: f64, t2322: f64, t2330: f64, t260: f64, t3430: f64, t3445: f64, t856: f64, t8736: f64, t8738: f64, t8741: f64, t8743: f64, t8754: f64, t8780: f64, t8813: f64, t8828: f64, t8856: f64, t8867: f64, t8868: f64, t8905: f64, t8908: f64, t8910: f64, t8925: f64, t8926: f64) -> (f64, f64, f64, f64) {
    let t8934 = t3435 * t2307;
    let t8937 = t6640 * t1370;
    let t8938 = t8937 * t2315;
    let t8941 = t3443 * t2292;
    let t8944 = -t8736 + t8738 - t8741 + 0.23392894490538584828e1_f64 * t856 * t8743 - 0.34631718211362927518e2_f64 * t2322 * t3445 + t260 * (t8780 + t8813 + t8868 + t8926) - 0.5848223622634646207e0_f64 * t3430 * t2330 + 0.19751673498613801407e-1_f64 * t260 * t8754 + t8828 + t8856 + t8867 + t8905 + t8908 + t8910 - t8925 + 0.11696447245269292414e1_f64 * t856 * t8934 + 0.10389515463408878255e3_f64 * t856 * t8938 - 0.35089341735807877242e1_f64 * t856 * t8941;
    (t8934, t8938, t8941, t8944)
}
