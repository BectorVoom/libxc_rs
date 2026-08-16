//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 811/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk811(t3311: f64, t459: f64, t2507: f64, t995: f64, t2528: f64, t987: f64, t3337: f64, t3314: f64, t4794: f64, t440: f64, t8: f64, t973: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8604 = t3311 * t459;
    let t8607 = t2507 * t995;
    let t8610 = t987 * t2528;
    let t8615 = t3337 * t459;
    let t8620 = t4794 * t3314;
    let t8621 = t8620 * t440;
    let t8624 = t973 * t8;
    (t8604, t8607, t8610, t8615, t8621, t8624)
}
