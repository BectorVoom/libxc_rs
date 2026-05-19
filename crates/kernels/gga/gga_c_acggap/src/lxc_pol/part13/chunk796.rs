//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 796/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk796<F: Float>(t1524: F, t599: F, t336: F, t578: F, t2020: F, t515: F, t7328: F, t7330: F, t7349: F, t7366: F, t7373: F, t7376: F, t7379: F, t8598: F, t8603: F, t8607: F, t8611: F, t8615: F, t8619: F) -> (F, F) {
    let t8621 = t599 * t1524;
    let t8622 = t336 * t8621;
    let t8623 = t578 * t8622;
    let t8625 = t2020 * t515;
    let t8627 = -t7328 + F::new(7.0) / F::new(144.0) * t7330 + F::cast_from(0.94344276868812456204e-3_f64) * t8598 - F::cast_from(0.18868855373762491241e-2_f64) * t8603 + F::cast_from(0.21437009059034868486e-3_f64) * t8607 + F::cast_from(0.10718504529517434243e-2_f64) * t8611 + F::cast_from(0.64311027177104605458e-3_f64) * t8615 + F::cast_from(0.10718504529517434243e-3_f64) * t7349 - F::cast_from(0.15724046144802076034e-3_f64) * t7366 + t7373 - t7376 + t7379 + F::cast_from(0.140078125e-1_f64) * t8619 - t8623 / F::new(384.0) + F::new(7.0) / F::new(288.0) * t8625;
    (t8622, t8627)
}
