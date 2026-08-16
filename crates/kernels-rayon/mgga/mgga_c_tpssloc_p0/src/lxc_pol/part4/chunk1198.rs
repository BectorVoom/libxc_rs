//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 1198/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk1198(t1365: f64, t6347: f64, t1307: f64, t1347: f64, t19631: f64, t1345: f64, t1348: f64, t1819: f64, t1821: f64, t19702: f64, t19708: f64, t19716: f64, t19719: f64, t5272: f64, t5278: f64, t5280: f64, t5283: f64, t546: f64, t548: f64, t6404: f64, t6408: f64, t6411: f64) -> f64 {
    let t19724 = t1365 * t6347;
    let t19725 = t19724 * t1307;
    let t19728 = t1347 * t19631;
    let t19731 = -12.0_f64 * t1345 * t6408 + 3.0_f64 * t1345 * t6411 + 3.0_f64 * t1348 * t6404 + 6.0_f64 * t1819 * t5283 + 6.0_f64 * t1821 * t5272 - t19702 * t548 - 24.0_f64 * t19708 * t5280 + 60.0_f64 * t19716 * t5278 - 24.0_f64 * t19719 * t5278 - 12.0_f64 * t19725 * t5278 + 3.0_f64 * t19728 * t546;
    t19731
}
