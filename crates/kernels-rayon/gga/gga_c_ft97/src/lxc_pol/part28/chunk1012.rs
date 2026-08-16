//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1012/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1012(t32399: f64, t6414: f64, t137356: f64, t137363: f64, t144704: f64, t144708: f64, t144714: f64, t144719: f64, t144725: f64, t144727: f64, t1564: f64, t25530: f64, t25558: f64, t32013: f64, t32021: f64, t379: f64, t5501: f64, t5504: f64, t7162: f64) -> f64 {
    let t144729 = t6414 * t32399;
    let t144731 = -4.0_f64 * t144704 - t137356 / 9.0_f64 - t137363 / 9.0_f64 + t144708 / 54.0_f64 + t25558 * t32021 / 9.0_f64 - t25558 * t32013 / 18.0_f64 - t5501 * t1564 * t144714 * t379 / 18.0_f64 - t144719 * t5504 / 18.0_f64 - t7162 * t25530 / 3.0_f64 - t144725 / 18.0_f64 - t144727 / 18.0_f64 + t144729 / 9.0_f64;
    t144731
}
