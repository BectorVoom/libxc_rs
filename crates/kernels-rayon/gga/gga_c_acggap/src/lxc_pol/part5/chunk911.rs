//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 911/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk911(t1161: f64, t134: f64, t3220: f64, t1170: f64, t3088: f64, t3371: f64, t3453: f64, t1095: f64, t1131: f64, t384: f64, t398: f64, t879: f64) -> (f64, f64, f64, f64, f64) {
    let t13850 = t1161 * t134 * t3220;
    let t13851 = t1170 * t13850;
    let t13860 = t3088 * t3371;
    let t13861 = t13860 * t3453;
    let t13881 = t384 * t398 * t1095 * t879 * t1131;
    (t13850, t13851, t13860, t13861, t13881)
}
