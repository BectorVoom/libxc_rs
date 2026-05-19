//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1362/1429 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1362<F: Float>(t36405: F, t36419: F, t35628: F, t35631: F, t35634: F, t35640: F, t35643: F, t35647: F, t35650: F, t35653: F, t35656: F, t35659: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t36420 = t36405 + t36419;
    let t36421 = F::cast_from(0.17379648562707520765e-3_f64) * t35628;
    let t36422 = F::cast_from(0.86898242813537603825e-4_f64) * t35631;
    let t36423 = F::cast_from(0.86898242813537603825e-4_f64) * t35634;
    let t36425 = F::cast_from(0.10862280351692200478e-4_f64) * t35640;
    let t36426 = F::cast_from(0.64377114884362441502e-6_f64) * t35643;
    let t36427 = F::cast_from(0.47522476538653377092e-5_f64) * t35647;
    let t36428 = F::cast_from(0.47522476538653377092e-5_f64) * t35650;
    let t36429 = F::cast_from(0.44241459320629195162e-6_f64) * t35653;
    let t36430 = F::cast_from(0.17379648562707520765e-3_f64) * t35656;
    let t36431 = F::cast_from(0.17379648562707520765e-3_f64) * t35659;
    (t36420, t36421, t36422, t36423, t36425, t36426, t36427, t36428, t36429, t36430, t36431)
}
