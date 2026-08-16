//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 653/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk653(t2244: f64, t2250: f64, t2291: f64, t2298: f64, t634: f64, t638: f64, t72: f64) -> f64 {
    let t2303 = 28.0_f64 / 9.0_f64 * t2291 * t2244 - 4.0_f64 / 3.0_f64 * t634 * t2250 + 28.0_f64 / 9.0_f64 * t2298 * t2244 + 4.0_f64 / 3.0_f64 * t638 * t2250;
    let t2304 = t72 * t2303;
    t2304
}
