//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1387/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1387(t58740: f64, t58752: f64, t1038: f64, t52446: f64, t52452: f64, t52591: f64, t52593: f64, t52596: f64, t52601: f64, t58415: f64, t58418: f64, t58421: f64, t58424: f64, t58428: f64, t58431: f64, t58435: f64) -> (f64, f64, f64) {
    let t58753 = t58740 + t58752;
    let t58754 = t1038 * t58753;
    let t58756 = -0.82785e-1_f64 * t58415 - 0.99342e0_f64 * t58418 - 0.82785e-1_f64 * t58421 + 0.198684e1_f64 * t58424 - 0.8585111111111111111e-1_f64 * t58428 - 0.89459259259259259259e0_f64 * t58431 - 0.301925e0_f64 * t58435 + 0.98115555555555555555e-1_f64 * t52591 - 0.44152e0_f64 * t52593 + 0.132456e1_f64 * t52596 + 0.22076e0_f64 * t52601 + 0.80513333333333333333e0_f64 * t52446 - 0.24154e1_f64 * t52452 + 0.258925e1_f64 * t58754;
    (t58753, t58754, t58756)
}
