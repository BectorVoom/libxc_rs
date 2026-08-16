//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 559/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk559(t23075: f64, t1882: f64, t5657: f64, t1328: f64, t1637: f64, t89: f64, t5724: f64, t1314: f64, t8232: f64, t376: f64, t5706: f64, t5637: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t23124 = 4.0_f64 / 27.0_f64 * t23075;
    let t23148 = t1882 * t5657;
    let t23152 = 4.0_f64 / 27.0_f64 * t89 * t1637 * t1328;
    let t23176 = t1882 * t5724;
    let t23183 = 4.0_f64 / 27.0_f64 * t8232 * t1314;
    let t23199 = t89 * t376 * t5706;
    let t23227 = t1882 * t5637;
    (t23124, t23148, t23152, t23176, t23183, t23199, t23227)
}
