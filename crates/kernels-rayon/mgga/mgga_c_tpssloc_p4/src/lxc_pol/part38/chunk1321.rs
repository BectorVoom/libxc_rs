//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1321/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1321(t110143: f64, t8226: f64, t110076: f64, t110078: f64, t110080: f64, t110089: f64, t110102: f64, t110103: f64, t110105: f64, t110503: f64, t110506: f64, t12808: f64, t1453: f64, t8128: f64, t8129: f64) -> f64 {
    let t110510 = t110143 * t8226;
    let t110517 = t8128 * t8129 * t12808 / 4.0_f64 + 22.0_f64 / 9.0_f64 * t110503 + t110506 + 10.0_f64 / 9.0_f64 * t8128 * t110089 * t1453 - 55.0_f64 / 27.0_f64 * t110510 + 2.0_f64 * t110076 + 20.0_f64 / 9.0_f64 * t110078 + 10.0_f64 / 27.0_f64 * t110080 + t110102 + 110.0_f64 / 27.0_f64 * t110103 + 40.0_f64 / 27.0_f64 * t110105;
    t110517
}
