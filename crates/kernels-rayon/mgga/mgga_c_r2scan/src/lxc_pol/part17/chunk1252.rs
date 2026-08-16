//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1252/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1252(t3245: f64, t3270: f64, t39030: f64, t3269: f64, t1115: f64, t3016: f64, t10667: f64, t11342: f64, t42389: f64, t3262: f64, t3465: f64, t43984: f64) -> (f64, f64, f64, f64) {
    let t44568 = t3270 * t39030 * t3245;
    let t44570 = t3269 * t44568 / 2.0_f64;
    let t44572 = t3270 * t1115 * t3016;
    let t44574 = 3.0_f64 / 4.0_f64 * t10667 * t44572;
    let t44576 = 3.0_f64 / 4.0_f64 * t42389 * t11342;
    let t44579 = 3.0_f64 / 4.0_f64 * t3262 * t3465 * t43984;
    (t44570, t44574, t44576, t44579)
}
