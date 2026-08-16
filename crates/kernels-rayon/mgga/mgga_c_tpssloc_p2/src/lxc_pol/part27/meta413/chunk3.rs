//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1710/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1710(t16428: f64, t3793: f64, t1380: f64, t16206: f64, t12267: f64, t1336: f64, t1383: f64, t16133: f64, t16136: f64, t16414: f64, t16416: f64, t16419: f64, t16423: f64, t1814: f64, t1838: f64, t1840: f64, t3773: f64, t3777: f64, t3898: f64, t3905: f64, t3907: f64, t3909: f64, t5230: f64, t5234: f64, t5339: f64, t5341: f64, t5344: f64, t544: f64) -> f64 {
    let t16429 = t16428 * t3793;
    let t16433 = t1380 * t16206;
    let t16435 = -t12267 * t1838 - 2.0_f64 * t1336 * t16133 - t1336 * t16136 - 2.0_f64 * t1336 * t16416 - t1336 * t16423 + 2.0_f64 * t1336 * t16429 - t1336 * t16433 + 2.0_f64 * t1383 * t5230 + t16414 * t544 - 2.0_f64 * t16419 * t5344 + t1814 * t3909 + t1840 * t3773 - 2.0_f64 * t3777 * t5339 - 2.0_f64 * t3777 * t5341 + 2.0_f64 * t3898 * t5234 - t3905 * t5234 - t3907 * t5234;
    t16435
}
