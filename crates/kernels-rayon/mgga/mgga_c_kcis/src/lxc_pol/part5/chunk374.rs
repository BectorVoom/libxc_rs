//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 374/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk374(t1307: f64, t1396: f64, t1395: f64, t1394: f64, t113: f64, t450: f64) -> (f64, f64, f64, f64) {
    let t1397 = t1396 * t1307;
    let t1398 = t1395 * t1397;
    let t1399 = t1394 * t1398;
    let t1401 = t113 * t450;
    (t1397, t1398, t1399, t1401)
}
