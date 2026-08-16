//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1991/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1991(t1484: f64, t2752: f64, t13487: f64, t2749: f64, t4303: f64, t868: f64, t4119: f64, t4233: f64, t829: f64, t16935: f64, t828: f64, t2745: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t57911 = t2752 * t1484;
    let t57912 = t57911 * t13487;
    let t57921 = t1484 * t2749;
    let t58009 = t4303 * t868;
    let t58071 = t4119 * t868;
    let t58300 = t829 * t4233;
    let t58345 = t16935 * t828;
    let t59580 = t1484 * t2745;
    (t57912, t57921, t58009, t58071, t58300, t58345, t59580)
}
