//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1340/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1340(t2927: f64, t2934: f64, t1062: f64, t8785: f64, t26252: f64, t26258: f64, t26278: f64, t26280: f64, t26284: f64, t26289: f64, t26293: f64, t26296: f64, t26300: f64, t26304: f64, t26306: f64) -> (f64, f64, f64) {
    let t26757 = t2927 * t2934;
    let t26760 = t1062 * t8785;
    let t26777 = 0.13734567901234567901e-1_f64 * t26252 + 0.12361111111111111111e0_f64 * t26258 - 0.61805555555555555555e-1_f64 * t26278 + 0.74166666666666666668e-1_f64 * t26280 - 0.22249999999999999999e0_f64 * t26284 + 0.22249999999999999999e0_f64 * t26289 - 0.18541666666666666666e-1_f64 * t26293 + 0.2225e0_f64 * t26296 - 0.33375e0_f64 * t26300 + 0.55625000000000000001e-1_f64 * t26304 - 0.74166666666666666668e-1_f64 * t26306;
    (t26757, t26760, t26777)
}
