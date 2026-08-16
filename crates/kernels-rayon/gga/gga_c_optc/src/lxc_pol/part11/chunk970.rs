//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 970/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk970(t1471: f64, t15401: f64, t11671: f64, t14885: f64, t14887: f64, t14889: f64, t17338: f64, t17342: f64, t17346: f64, t17350: f64, t17354: f64, t17358: f64, t8871: f64) -> (f64, f64) {
    let t17764 = t15401 * t1471;
    let t17777 = -t8871 - 0.2283111111111111111e-1_f64 * t11671 + 0.11415555555555555555e-1_f64 * t14885 - 0.34246666666666666665e-1_f64 * t14887 + 0.17123333333333333333e-1_f64 * t14889 - 0.19025925925925925925e-1_f64 * t17338 + 0.68493333333333333331e-1_f64 * t17342 - 0.34246666666666666665e-1_f64 * t17346 - 0.10274e0_f64 * t17350 + 0.10274e0_f64 * t17354 - 0.17123333333333333333e-1_f64 * t17358;
    (t17764, t17777)
}
