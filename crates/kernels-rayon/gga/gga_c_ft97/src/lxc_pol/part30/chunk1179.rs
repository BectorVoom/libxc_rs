//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1179/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1179(t25462: f64, t35810: f64, t143007: f64, t143008: f64, t143018: f64, t143024: f64, t1466: f64, t154083: f64, t154463: f64, t154503: f64, t154550: f64, t154851: f64, t193: f64, t28835: f64, t29416: f64, t34056: f64, t36093: f64, t6225: f64, t7618: f64) -> f64 {
    let t155010 = t25462 * t35810;
    let t155028 = t143007 + t155010 / 54.0_f64 - t143008 / 9.0_f64 - 2.0_f64 * t154083 + t29416 * t7618 / 3.0_f64 + t143018 / 54.0_f64 - t143024 / 27.0_f64 - 2.0_f64 * t154550 + 4.0_f64 * t154463 - 4.0_f64 * t154503 - 2.0_f64 / 3.0_f64 * t1466 * t193 * t28835 * t34056 - t36093 * t6225 / 3.0_f64 - 2.0_f64 * t154851;
    t155028
}
