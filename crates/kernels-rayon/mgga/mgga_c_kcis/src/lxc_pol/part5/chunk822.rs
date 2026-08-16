//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 822/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk822(t1009: f64, t6544: f64, t2836: f64, t300: f64, t3247: f64, t6302: f64, t6432: f64, t6484: f64, t6489: f64, t6494: f64, t6499: f64, t6502: f64, t6506: f64, t6510: f64, t979: f64) -> (f64, f64) {
    let t6545 = t6544 * t1009;
    let t6548 = 0.66725e-1_f64 * t979 * t6302 + 0.24872916666666666666e-2_f64 * t6484 + 0.49745833333333333332e-2_f64 * t6489 - 0.33163888888888888888e-2_f64 * t6494 - 0.55273148148148148147e-3_f64 * t6499 + 0.33163888888888888888e-2_f64 * t6502 + 0.16581944444444444444e-2_f64 * t6506 + 0.27636574074074074073e-2_f64 * t6510 - t3247 + t6432 * t300 + 0.890445125e-2_f64 * t2836 * t6302 - 0.66725e-1_f64 * t979 * t6545;
    (t6545, t6548)
}
