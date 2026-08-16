//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1090/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1090(t10488: f64, t2189: f64, t10491: f64, t7743: f64, t3325: f64, t7766: f64, t3331: f64, t10498: f64, t1203: f64, t3330: f64, t3481: f64, t3227: f64, t3444: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t26875 = t10488 * t2189;
    let t26877 = 4.0_f64 * t10491 * t7743;
    let t26879 = 2.0_f64 * t3325 * t7766;
    let t26880 = t2189 * t3331;
    let t26882 = 6.0_f64 * t10498 * t26880;
    let t26883 = t7766 * t1203;
    let t26885 = 4.0_f64 * t3330 * t26883;
    let t26886 = t2189 * t3481;
    let t26888 = 2.0_f64 * t3330 * t26886;
    let t26889 = t3227 * t3444;
    (t26875, t26877, t26879, t26880, t26882, t26883, t26885, t26886, t26888, t26889)
}
