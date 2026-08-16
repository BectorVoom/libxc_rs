//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1406/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1406(t1081: f64, t115009: f64, t119719: f64, t121789: f64, t121837: f64, t1877: f64, t24191: f64, t24339: f64, t2522: f64, t25892: f64, t25898: f64, t25905: f64, t25927: f64, t25934: f64, t25945: f64, t31430: f64, t31434: f64, t31496: f64, t33466: f64, t33539: f64, t7649: f64, t8566: f64, t92319: f64) -> f64 {
    let t122012 = t1877 * t33466 * t1081 / 2.0_f64 - t1877 * t31434 * t25934 / 2.0_f64 - t1877 * t24339 * t33539 / 2.0_f64 + 3.0_f64 * t24191 * t25927 * t121837 - t1877 * t31434 * t25945 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t92319 * t31496 - 3.0_f64 / 2.0_f64 * t115009 * t25898 + 3.0_f64 / 2.0_f64 * t2522 * t31430 * t7649 + 3.0_f64 / 2.0_f64 * t2522 * t8566 * t25905 + 3.0_f64 * t121789 * t25892 - 3.0_f64 / 2.0_f64 * t24191 * t119719;
    t122012
}
