//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 930/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk930(t385: f64, t13593: f64, t13759: f64, t67: f64, t1389: f64, t381: f64, t916: f64, t1286: f64, t3777: f64, t1284: f64, t13423: f64, t1280: f64, t1287: f64, t13394: f64, t340: f64, t379: f64, t382: f64, t4134: f64, t4144: f64, t4148: f64, t6141: f64, t6142: f64) -> (f64, f64, f64, f64) {
    let t386 = t385 < -0.66725e-1_f64;
    let t13761 = t67 * (t13593 + t13759);
    let t13776 = 1.0_f64 / t381 / t916 / t1389;
    let t13777 = t3777 * t1286;
    let t13778 = t13776 * t13777;
    let t13785 = t1284 * t13423;
    let t13790 = piecewise3(t386, 0.0_f64, 10.0_f64 / 9.0_f64 * t340 * t13761 * t382 - 10.0_f64 / 9.0_f64 * t340 * t4134 * t1287 + 40.0_f64 / 27.0_f64 * t340 * t1280 * t4144 - 10.0_f64 / 9.0_f64 * t340 * t1280 * t4148 - 280.0_f64 / 243.0_f64 * t340 * t379 * t13778 + 40.0_f64 / 27.0_f64 * t6141 * t6142 * t13394 - 10.0_f64 / 27.0_f64 * t340 * t379 * t13785);
    (t13777, t13778, t13785, t13790)
}
