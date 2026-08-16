//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 848/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk848(t127: f64, t16370: f64, t6: f64, t161: f64, t16324: f64, t1271: f64, t4649: f64, t162: f64, t1256: f64, t13174: f64, t2034: f64, t3353: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t16372 = t6 * t16370 * t127;
    let t16373 = t161 * t16372;
    let t16376 = t16324 * t127;
    let t16377 = t161 * t16376;
    let t16380 = t4649 * t1271;
    let t16381 = t16380 * t127;
    let t16382 = t162 * t16381;
    let t16385 = t13174 * t1256;
    let t16386 = t2034 * t16385;
    let t16389 = t3353 * t4649;
    (t16372, t16373, t16376, t16377, t16380, t16381, t16382, t16385, t16386, t16389)
}
