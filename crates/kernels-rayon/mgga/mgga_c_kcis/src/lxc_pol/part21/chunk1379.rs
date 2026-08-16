//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1379/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1379(t1281: f64, t28250: f64, t10498: f64, t1203: f64, t28002: f64, t27987: f64, t3481: f64, t26950: f64, t5036: f64, t26880: f64, t46026: f64, t10491: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t97494 = t28250 * t1281;
    let t97499 = 12.0_f64 * t10498 * t28002 * t1203;
    let t97500 = t27987 * t3481;
    let t97501 = t5036 * t26950;
    let t97503 = 6.0_f64 * t46026 * t26880;
    let t97505 = 4.0_f64 * t10491 * t28002;
    (t97494, t97499, t97500, t97501, t97503, t97505)
}
