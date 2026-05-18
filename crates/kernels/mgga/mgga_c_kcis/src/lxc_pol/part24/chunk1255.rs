//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1255/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1255<F: Float>(t2822: F, t28935: F, t28105: F, t92693: F, t96736: F, t28111: F, t96735: F, t1020: F, t19599: F, t26760: F, t26955: F, t27070: F, t29094: F, t97026: F, t97028: F, t97030: F, t97031: F, t97051: F, t97060: F) -> (F, F, F, F, F) {
    let t100494 = t2822 * t28935;
    let t100497 = t92693 * t96736 * t28105;
    let t100501 = t96735 * t96736 * t28111;
    let t100505 = t1020 * t26760 * t19599;
    let t100507 = -F::new(0.13913205078125e-3) * t27070 * t29094 - t97026 - t97028 - t97030 - t97031 + t97051 - t97060 + F::new(0.10317654320987654321e-2) * t100494 - F::new(0.30918233506944444444e-4) * t26955 * t100497 - F::new(0.92754700520833333333e-4) * t26955 * t100501 - F::new(0.11607361111111111111e-2) * t100505;
    (t100494, t100497, t100501, t100505, t100507)
}
