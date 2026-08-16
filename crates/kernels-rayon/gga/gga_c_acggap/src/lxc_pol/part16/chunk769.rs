//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 769/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk769(t1524: f64, t599: f64, t336: f64, t578: f64, t2020: f64, t515: f64, t7328: f64, t7330: f64, t7349: f64, t7366: f64, t7373: f64, t7376: f64, t7379: f64, t8598: f64, t8603: f64, t8607: f64, t8611: f64, t8615: f64, t8619: f64) -> (f64, f64, f64) {
    let t8621 = t599 * t1524;
    let t8622 = t336 * t8621;
    let t8623 = t578 * t8622;
    let t8625 = t2020 * t515;
    let t8627 = -t7328 + 7.0_f64 / 144.0_f64 * t7330 + 0.94344276868812456204e-3_f64 * t8598 - 0.18868855373762491241e-2_f64 * t8603 + 0.21437009059034868486e-3_f64 * t8607 + 0.10718504529517434243e-2_f64 * t8611 + 0.64311027177104605458e-3_f64 * t8615 + 0.10718504529517434243e-3_f64 * t7349 - 0.15724046144802076034e-3_f64 * t7366 + t7373 - t7376 + t7379 + 0.140078125e-1_f64 * t8619 - t8623 / 384.0_f64 + 7.0_f64 / 288.0_f64 * t8625;
    (t8622, t8625, t8627)
}
