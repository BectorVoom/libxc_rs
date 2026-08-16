//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1112/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1112(t43: f64, t11698: f64, t11700: f64, t11702: f64, t11704: f64, t14919: f64, t11607: f64, t1361: f64, t14810: f64, t1690: f64, t1694: f64, t19461: f64, t234: f64, t2861: f64, t2868: f64, t35: f64, t3996: f64, t5445: f64, t5450: f64, t5455: f64, t595: f64, t818: f64, t821: f64, t824: f64, t886: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t44 = t43 <= zeta_threshold;
    let t19914 = 192.0_f64 * t11698;
    let t19915 = 24.0_f64 * t11700;
    let t19916 = 64.0_f64 * t11702;
    let t19917 = 0.11696447245269292414e1_f64 * t11704;
    let t19918 = 32.0_f64 * t14919;
    let t19942 = piecewise3(t44, 0.0_f64, 40.0_f64 / 81.0_f64 * t11607 * t1690 * t818 - 64.0_f64 / 27.0_f64 * t3996 * t19461 - 8.0_f64 / 27.0_f64 * t5445 * t824 + 32.0_f64 / 9.0_f64 * t886 * t35 * t595 + 16.0_f64 / 9.0_f64 * t1361 * t821 - 16.0_f64 / 3.0_f64 * t1361 * t2868 - 8.0_f64 / 27.0_f64 * t2861 * t1694 * t818 + 8.0_f64 / 9.0_f64 * t886 * t5455 * t234 + 4.0_f64 / 9.0_f64 * t5450 * t824 + t14810);
    (t19914, t19915, t19916, t19917, t19918, t19942)
}
