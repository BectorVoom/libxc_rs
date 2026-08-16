//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1138/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1138(t14422: f64, t285: f64, t25: f64, t4973: f64, t291: f64, t992: f64, t330: f64, t737: f64, t14394: f64, t14397: f64, t14404: f64, t14409: f64, t14415: f64, t4974: f64, t9608: f64, t9611: f64, t9614: f64, t9620: f64, t9623: f64, t984: f64) -> f64 {
    let t14423 = t285 * t14422;
    let t14425 = t25 * t4973;
    let t14427 = t285 * t14425 / 144.0_f64;
    let t14430 = t992 * t291;
    let t14431 = t14430 * t330;
    let t14432 = t737 * t14431;
    let t14435 = 11.0_f64 / 324.0_f64 * t9608 + t14394 * t14397 / 72.0_f64 + t14394 * t14404 / 72.0_f64 - t14394 * t14409 / 108.0_f64 - t285 * t14415 / 96.0_f64 + t9611 / 144.0_f64 + t9614 / 216.0_f64 + t9620 / 54.0_f64 - t9623 / 288.0_f64 + t14423 / 432.0_f64 - t14427 + t984 * t4974 / 18.0_f64 + t285 * t14432 / 144.0_f64;
    t14435
}
