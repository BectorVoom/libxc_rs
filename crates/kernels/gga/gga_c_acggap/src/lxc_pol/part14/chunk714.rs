//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 714/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk714<F: Float>(t578: F, t8622: F, t2020: F, t515: F, t7328: F, t7330: F, t7349: F, t7366: F, t7373: F, t7376: F, t7379: F, t8598: F, t8603: F, t8607: F, t8611: F, t8615: F, t8619: F) -> (F, F) {
    let t8623 = t578 * t8622;
    let t8625 = t2020 * t515;
    let t8627 = -t7328 + 7.0 / 144.0 * t7330 + 0.94344276868812456204e-3 * t8598 - 0.18868855373762491241e-2 * t8603 + 0.21437009059034868486e-3 * t8607 + 0.10718504529517434243e-2 * t8611 + 0.64311027177104605458e-3 * t8615 + 0.10718504529517434243e-3 * t7349 - 0.15724046144802076034e-3 * t7366 + t7373 - t7376 + t7379 + 0.140078125e-1 * t8619 - t8623 / 384.0 + 7.0 / 288.0 * t8625;
    (t8625, t8627)
}
