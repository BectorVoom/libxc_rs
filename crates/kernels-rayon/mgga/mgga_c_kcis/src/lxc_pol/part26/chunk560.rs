//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 560/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk560(t5835: f64, t5866: f64, t1360: f64, t1404: f64, t1455: f64, t1924: f64, t1979: f64, t4018: f64, t4019: f64, t4021: f64, t4023: f64, t4059: f64, t486: f64, t510: f64, t538: f64, t5623: f64, t5787: f64, t5789: f64, t5793: f64, t5796: f64, t5799: f64, t5801: f64, t5805: f64, t5808: f64) -> (f64, f64) {
    let t5867 = t5835 + t5866;
    let t5869 = t4018 + 0.23426533963880895498e-2_f64 * t4019 + 0.46853067927761790996e-2_f64 * t4021 + 0.23426533963880895498e-2_f64 * t5787 + 0.46853067927761790996e-2_f64 * t4023 * t5789 + 0.46853067927761790996e-2_f64 * t1404 * t5793 + 0.46853067927761790996e-2_f64 * t4059 * t5796 + 0.46853067927761790996e-2_f64 * t5799 + 0.46853067927761790996e-2_f64 * t1404 * t5801 + 0.14055920378328537299e-1_f64 * t510 * t5805 - 0.46853067927761790996e-2_f64 * t510 * t5808 - t5623 * t538 - t1924 * t1455 - t1360 * t1979 - t486 * t5867;
    (t5867, t5869)
}
