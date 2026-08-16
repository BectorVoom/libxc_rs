//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 994/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk994(t12203: f64, t3269: f64, t11497: f64, t3465: f64, t3262: f64, t1115: f64, t3270: f64, t910: f64, t10667: f64, t11342: f64, t11506: f64, t11509: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12204 = t3269 * t12203;
    let t12205 = 5.0_f64 / 16.0_f64 * t12204;
    let t12206 = t3465 * t11497;
    let t12207 = t3262 * t12206;
    let t12208 = 3.0_f64 / 4.0_f64 * t12207;
    let t12210 = t3270 * t1115 * t910;
    let t12211 = t10667 * t12210;
    let t12212 = 3.0_f64 / 4.0_f64 * t12211;
    let t12213 = t11506 * t11342;
    let t12214 = 3.0_f64 / 4.0_f64 * t12213;
    let t12215 = t3465 * t11509;
    (t12204, t12205, t12206, t12207, t12208, t12210, t12211, t12212, t12213, t12214, t12215)
}
