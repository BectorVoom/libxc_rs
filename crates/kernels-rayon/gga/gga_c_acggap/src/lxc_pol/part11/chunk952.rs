//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 952/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk952(t31662: f64, t1086: f64, t7605: f64, t1998: f64, t3531: f64, t7528: f64, t7637: f64, t2109: f64, t7610: f64, t1980: f64, t31025: f64, t7458: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t31663 = 0.38586616306262763276e-2_f64 * t31662;
    let t31676 = t7605 * t1086;
    let t31680 = t1998 * t3531;
    let t31682 = t7637 * t7528;
    let t31684 = t7610 * t2109;
    let t31687 = t1980 * t7458 * t31025;
    (t31663, t31676, t31680, t31682, t31684, t31687)
}
