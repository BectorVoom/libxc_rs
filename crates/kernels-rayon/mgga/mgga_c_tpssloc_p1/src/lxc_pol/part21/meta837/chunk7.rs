//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2985/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2985(t10390: f64, t10403: f64, t10480: f64, t10883: f64, t13985: f64, t17670: f64, t17677: f64, t17705: f64, t17712: f64, t17980: f64, t2776: f64, t3041: f64, t3071: f64, t3121: f64, t3132: f64, t42347: f64, t42354: f64, t42358: f64, t42496: f64, t4582: f64, t49940: f64, t49945: f64, t49957: f64, t49959: f64, t49964: f64, t49966: f64, t5873: f64, t5909: f64) -> f64 {
    let t62398 = t10480 * t4582 * t17712 * t13985 / 512.0_f64 + t42354 * t17980 / 1536.0_f64 + t10883 * t4582 * t17670 * t3121 / 3072.0_f64 + 7.0_f64 / 1536.0_f64 * t42347 * t4582 * t17670 * t3132 - t42358 * t4582 * t17670 * t3041 / 3072.0_f64 + t49940 / 1152.0_f64 - t49945 / 1728.0_f64 - t10403 * t3071 * t5873 * t2776 / 1152.0_f64 + t49957 / 1152.0_f64 - t49959 / 2304.0_f64 + t49964 / 1152.0_f64 + t49966 / 1728.0_f64 + t10390 * t17677 / 1152.0_f64 - t42496 * t5909 / 216.0_f64 + t10390 * t17705 / 1152.0_f64;
    t62398
}
