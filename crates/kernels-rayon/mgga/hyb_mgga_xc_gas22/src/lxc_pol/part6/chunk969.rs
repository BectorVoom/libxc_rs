//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 969/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk969(t1333: f64, t2187: f64, t2190: f64, t1359: f64, t1371: f64, t2246: f64, t2285: f64, t2307: f64, t2315: f64, t3386: f64, t3399: f64, t3419: f64, t6673: f64, t6729: f64, t821: f64, t840: f64, t849: f64, t8869: f64, t8901: f64, t8905: f64, t8908: f64, t8910: f64, t8911: f64, t8916: f64) -> (f64, f64, f64) {
    let t8923 = t1333 * t2187;
    let t8925 = 2.0_f64 * t8923 * t2190;
    let t8926 = 0.5848223622634646207e0_f64 * t840 * t8869 + 1.0_f64 * t6673 * t1359 + 2.0_f64 * t2246 * t3386 + 1.0_f64 * t821 * t8901 - t8905 - t8908 - t8910 + 0.11696447245269292414e1_f64 * t8911 * t849 + 0.5848223622634646207e0_f64 * t3399 * t2307 + 0.17315859105681463759e2_f64 * t8916 * t2315 + 0.5848223622634646207e0_f64 * t6729 * t1371 + 0.11696447245269292414e1_f64 * t2285 * t3419 + t8925;
    (t8923, t8925, t8926)
}
