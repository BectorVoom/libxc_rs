//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 711/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk711(t1396: f64, t4124: f64, t4123: f64, t1464: f64, t1489: f64, t1497: f64) -> (f64, f64, f64, f64) {
    let t4125 = t1396 * t4124;
    let t4126 = t4123 * t4125;
    let t4127 = t1464 * t4126;
    let t4129 = t1489 * t1497;
    (t4125, t4126, t4127, t4129)
}
