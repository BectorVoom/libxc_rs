//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1359/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1359(t36405: f64, t36419: f64, t35628: f64, t35631: f64, t35634: f64, t35640: f64, t35643: f64, t35647: f64, t35650: f64, t35653: f64, t35656: f64, t35659: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t36420 = t36405 + t36419;
    let t36421 = 0.17379648562707520765e-3_f64 * t35628;
    let t36422 = 0.86898242813537603825e-4_f64 * t35631;
    let t36423 = 0.86898242813537603825e-4_f64 * t35634;
    let t36425 = 0.10862280351692200478e-4_f64 * t35640;
    let t36426 = 0.64377114884362441502e-6_f64 * t35643;
    let t36427 = 0.47522476538653377092e-5_f64 * t35647;
    let t36428 = 0.47522476538653377092e-5_f64 * t35650;
    let t36429 = 0.44241459320629195162e-6_f64 * t35653;
    let t36430 = 0.17379648562707520765e-3_f64 * t35656;
    let t36431 = 0.17379648562707520765e-3_f64 * t35659;
    (t36420, t36421, t36422, t36423, t36425, t36426, t36427, t36428, t36429, t36430, t36431)
}
