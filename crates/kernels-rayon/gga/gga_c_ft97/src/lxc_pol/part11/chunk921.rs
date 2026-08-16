//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 921/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk921(t1827: f64, t8232: f64, t1882: f64, t8468: f64, t1637: f64, t1843: f64, t89: f64, t11587: f64, t12020: f64, t1825: f64, t1871: f64, t1901: f64, t3193: f64, t38300: f64, t38304: f64, t38379: f64, t38648: f64, t38665: f64, t38930: f64, t446: f64, t83: f64, t8377: f64, t8539: f64) -> f64 {
    let t38983 = t8232 * t1827;
    let t38988 = t1882 * t8468;
    let t38991 = t89 * t1637 * t1843;
    let t39000 = 8.0_f64 / 3.0_f64 * t1901 * t3193 * t12020 * t38930 - 8.0_f64 / 9.0_f64 * t1901 * t11587 * t8377 - 4.0_f64 / 3.0_f64 * t446 * t83 * t38304 - 4.0_f64 / 3.0_f64 * t446 * t83 * t38300 + 2.0_f64 * t446 * t83 * t38648 - 16.0_f64 / 9.0_f64 * t38983 - 2.0_f64 * t446 * t83 * t38379 - 8.0_f64 / 3.0_f64 * t38988 + 8.0_f64 / 9.0_f64 * t38991 + 4.0_f64 * t446 * t83 * t38665 - 8.0_f64 * t446 * t1871 * t1825 * t8539;
    t39000
}
