//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 388/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk388(t136: f64, t1845: f64, t191: f64, t617: f64, t636: f64, t1044: f64, t147: f64, t19: f64, t648: f64, t118: f64, t1388: f64, t129: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1846 = t1845 * t136;
    let t1847 = t1846 * t191;
    let t1850 = t617 * t636;
    let t1854 = t1044 * t19 * t147;
    let t1855 = t1854 * t648;
    let t1860 = t1388 * t118;
    let t1861 = t1860 * t129;
    (t1846, t1847, t1850, t1854, t1855, t1860, t1861)
}
