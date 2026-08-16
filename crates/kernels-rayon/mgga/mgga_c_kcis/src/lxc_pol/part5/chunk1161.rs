//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1161/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1161(t6509: f64, t9386: f64, t1133: f64, t6272: f64, t9432: f64, t3210: f64, t3200: f64, t19396: f64, t4555: f64, t19399: f64, t4546: f64, t13172: f64) -> (f64, f64, f64, f64, f64) {
    let t19550 = t9386 * t6509;
    let t19552 = t6272 * t1133;
    let t19553 = t9432 * t19552;
    let t19554 = t3210 * t19553;
    let t19555 = t3200 * t19554;
    let t19557 = t4555 * t19396;
    let t19558 = t3210 * t19557;
    let t19559 = t3200 * t19558;
    let t19561 = t4546 * t19399;
    let t19562 = t3210 * t19561;
    let t19563 = t13172 * t19562;
    (t19550, t19552, t19555, t19559, t19563)
}
