//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1168/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1168(t43463: f64, t43448: f64, t43453: f64, t43457: f64, t43460: f64, t43466: f64, t43471: f64, t43474: f64, t43478: f64, t43483: f64, t43487: f64, t43490: f64, t43493: f64, t43498: f64) -> f64 {
    let t44757 = 8.0_f64 / 27.0_f64 * t43463;
    let t44767 = -t43448 / 9.0_f64 - 4.0_f64 / 3.0_f64 * t43453 - 4.0_f64 / 3.0_f64 * t43457 + 2.0_f64 / 27.0_f64 * t43460 + t44757 + 4.0_f64 / 9.0_f64 * t43466 + 20.0_f64 / 81.0_f64 * t43471 - 10.0_f64 / 27.0_f64 * t43474 + 4.0_f64 / 3.0_f64 * t43478 - 40.0_f64 / 243.0_f64 * t43483 - t43487 / 18.0_f64 - 8.0_f64 / 27.0_f64 * t43490 + 20.0_f64 / 243.0_f64 * t43493 + 20.0_f64 / 27.0_f64 * t43498;
    t44767
}
