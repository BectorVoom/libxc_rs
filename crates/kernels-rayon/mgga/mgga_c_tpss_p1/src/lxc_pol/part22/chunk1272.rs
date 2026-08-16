//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1272/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1272(t1707: f64, t17993: f64, t18006: f64, t19767: f64, t20471: f64, t20475: f64, t20479: f64, t20483: f64, t20488: f64, t20492: f64, t20494: f64, t20498: f64, t20503: f64, t20506: f64, t5568: f64, t5571: f64, t6348: f64, t6351: f64) -> f64 {
    let t20508 = -t1707 * t20506 + t17993 * t6348 - 2.0_f64 * t18006 * t20479 - 2.0_f64 * t19767 * t20483 + t19767 * t20494 + 2.0_f64 * t20471 * t5571 + 2.0_f64 * t20475 * t5571 + t20488 * t5571 + t20492 * t5571 + 2.0_f64 * t20498 * t5571 + t20503 * t5571 - t5568 * t6351;
    t20508
}
