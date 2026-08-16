//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 958/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk958(t284: f64, t8753: f64, t3419: f64, t847: f64, t1371: f64, t2306: f64, t2291: f64, t3422: f64, t2314: f64, t3418: f64, t1370: f64, t6669: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8754 = t8753 * t284;
    let t8760 = t3419 * t847;
    let t8763 = t1371 * t2306;
    let t8766 = t3422 * t2291;
    let t8769 = t3418 * t2314;
    let t8770 = t8769 * t847;
    let t8773 = t3422 * t2306;
    let t8776 = t1370 * t6669;
    (t8754, t8760, t8763, t8766, t8769, t8770, t8773, t8776)
}
