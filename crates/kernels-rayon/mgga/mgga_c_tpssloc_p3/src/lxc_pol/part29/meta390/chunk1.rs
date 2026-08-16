//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1597/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1597(t14841: f64, t3404: f64, t1164: f64, t1098: f64, t4737: f64, t1119: f64, t3308: f64, t4740: f64, t1657: f64, t3312: f64, t3316: f64, t11282: f64, t1694: f64) -> (f64, f64, f64, f64, f64) {
    let t14842 = t14841 * t3404;
    let t14844 = 0.10389515463408878255e3_f64 * t1164 * t14842;
    let t14845 = t4737 * t1098;
    let t14847 = 2.0_f64 * t14845 * t1119;
    let t14849 = 1.0_f64 * t4740 * t3308;
    let t14850 = t1657 * t3312;
    let t14852 = 0.16081979498692535067e2_f64 * t14850 * t3316;
    let t14853 = t11282 * t1694;
    (t14844, t14847, t14849, t14852, t14853)
}
