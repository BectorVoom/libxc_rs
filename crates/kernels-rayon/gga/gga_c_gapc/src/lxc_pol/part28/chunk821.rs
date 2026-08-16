//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 821/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk821(t3094: f64, t9359: f64, t5541: f64, t612: f64, t1671: f64, t5544: f64, t9326: f64, t9331: f64, t9334: f64, t9337: f64, t9339: f64, t9341: f64, t9344: f64, t9346: f64, t9349: f64, t9351: f64, t9354: f64, t9357: f64) -> (f64, f64, f64) {
    let t9360 = t3094 * t9359;
    let t9362 = t5541 * t612;
    let t9363 = t1671 * t5544;
    let t9364 = t9362 * t9363;
    let t9366 = -0.11795371371935910947e-5_f64 * t9326 - 0.36954560225358884233e-5_f64 * t9331 + 0.7588373973867992891e-7_f64 * t9334 - 0.13492128925537291361e-6_f64 * t9337 - 0.15176747947735985782e-6_f64 * t9339 + 0.26984257851074582721e-6_f64 * t9341 + 0.4637672555408563478e-4_f64 * t9344 - 0.4637672555408563478e-4_f64 * t9346 - 0.86880925264517213544e-4_f64 * t9349 - 0.17376185052903442709e-3_f64 * t9351 + 0.14480154210752868924e-5_f64 * t9354 - 0.86880925264517213544e-4_f64 * t9357 + 0.14480154210752868924e-5_f64 * t9360 + 0.50680539737635041234e-4_f64 * t9364;
    (t9360, t9364, t9366)
}
