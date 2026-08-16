//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1144/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1144(t2391: f64, t2399: f64, t2382: f64, t214: f64, t2383: f64, t23605: f64, t23608: f64, t23612: f64, t23614: f64, t23616: f64, t23653: f64, t23655: f64, t23670: f64, t23673: f64, t23676: f64, t23679: f64) -> (f64, f64, f64, f64, f64) {
    let t23839 = t2391 * t2391;
    let t23840 = t2399 * t23839;
    let t23842 = t2382 * t23839;
    let t23844 = f64::powf(t214, -0.25e1_f64);
    let t23845 = t2383 * t2383;
    let t23846 = t23844 * t23845;
    let t23859 = -8.0_f64 * t23605 + 8.0_f64 * t23670 - 2.0_f64 / 3.0_f64 * t23608 - 8.0_f64 / 9.0_f64 * t23673 - 20.0_f64 / 9.0_f64 * t23676 + 8.0_f64 * t23612 - 12.0_f64 * t23679 + 16.0_f64 / 9.0_f64 * t23614 + 8.0_f64 / 3.0_f64 * t23616 - 8.0_f64 / 3.0_f64 * t23653 + 8.0_f64 / 9.0_f64 * t23655;
    (t23840, t23842, t23845, t23846, t23859)
}
