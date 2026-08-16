//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1255/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1255(t2822: f64, t28935: f64, t28105: f64, t92693: f64, t96736: f64, t28111: f64, t96735: f64, t1020: f64, t19599: f64, t26760: f64, t26955: f64, t27070: f64, t29094: f64, t97026: f64, t97028: f64, t97030: f64, t97031: f64, t97051: f64, t97060: f64) -> (f64, f64, f64, f64, f64) {
    let t100494 = t2822 * t28935;
    let t100497 = t92693 * t96736 * t28105;
    let t100501 = t96735 * t96736 * t28111;
    let t100505 = t1020 * t26760 * t19599;
    let t100507 = -0.13913205078125e-3_f64 * t27070 * t29094 - t97026 - t97028 - t97030 - t97031 + t97051 - t97060 + 0.10317654320987654321e-2_f64 * t100494 - 0.30918233506944444444e-4_f64 * t26955 * t100497 - 0.92754700520833333333e-4_f64 * t26955 * t100501 - 0.11607361111111111111e-2_f64 * t100505;
    (t100494, t100497, t100501, t100505, t100507)
}
