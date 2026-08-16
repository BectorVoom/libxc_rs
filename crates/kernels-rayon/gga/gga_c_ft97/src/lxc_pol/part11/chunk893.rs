//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 893/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk893(t11262: f64, t1526: f64, t7713: f64, t7763: f64, t81: f64, t11269: f64, t1527: f64, t1528: f64, t3088: f64, t38310: f64, t38313: f64, t38316: f64, t7734: f64, t7745: f64, t7765: f64, t7794: f64, t7811: f64, t7815: f64, t8199: f64) -> f64 {
    let t38319 = t1526 * t11262 * t7713;
    let t38327 = t81 * t7763;
    let t38339 = t38310 / 18.0_f64 - t38313 / 6.0_f64 - t38316 / 12.0_f64 - t38319 / 9.0_f64 - t1526 * t1527 * t7811 / 4.0_f64 - t1526 * t3088 * t7794 / 3.0_f64 - 7.0_f64 / 27.0_f64 * t1526 * t11269 * t38327 * t7765 - t1526 * t1527 * t7815 / 4.0_f64 - t1526 * t1527 * t1528 * t7745 / 12.0_f64 + t7734 + t8199;
    t38339
}
