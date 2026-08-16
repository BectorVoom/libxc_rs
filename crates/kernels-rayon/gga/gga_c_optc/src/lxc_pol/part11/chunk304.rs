//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 304/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk304(t127: f64, t1271: f64, t6: f64, t161: f64, t1256: f64, t141: f64, t659: f64) -> (f64, f64, f64, f64) {
    let t1273 = t6 * t1271 * t127;
    let t1274 = t161 * t1273;
    let t1277 = t141 * t1256;
    let t1278 = t659 * t1277;
    (t1273, t1274, t1277, t1278)
}
