//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 735/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk735(t1506: f64, t4310: f64, t1615: f64, t622: f64, t3793: f64, t3795: f64, t3799: f64, t3803: f64, t3807: f64, t1559: f64, t1563: f64, t1562: f64, t597: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4311 = t1506 * t4310;
    let t4312 = t1615 * t1615;
    let t4313 = t622 * t622;
    let t4314 = 1.0_f64 / t4313;
    let t4315 = t4312 * t4314;
    let t4318 = 0.22831111111111111111e-1_f64 * t3793;
    let t4323 = t4318 + 0.11415555555555555555e-1_f64 * t3795 - 0.11415555555555555555e-1_f64 * t3799 + 0.34246666666666666666e-1_f64 * t3803 - 0.17123333333333333333e-1_f64 * t3807;
    let t4326 = t1559 * t1563;
    let t4329 = t1562 * t597;
    (t4311, t4312, t4313, t4314, t4315, t4318, t4323, t4326, t4329)
}
