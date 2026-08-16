//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 937/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk937(t45905: f64, t1: f64, t44766: f64, t787: f64, t13555: f64, t4614: f64, t833: f64, t10811: f64, t10978: f64, t2028: f64, t44070: f64, t44084: f64, t44088: f64, t44114: f64, t44117: f64, t44120: f64, t45877: f64, t45882: f64, t45885: f64, t45887: f64, t45888: f64, t45892: f64, t45894: f64, t45898: f64, t45900: f64, t45903: f64) -> f64 {
    let t45906 = 0.89376224879626066674e-1_f64 * t45905;
    let t45908 = t787 * t44766 * t1;
    let t45913 = 0.15337170381568299871e2_f64 * t833 * t4614 * t13555;
    let t45915 = 0.85801175884441024006e1_f64 * t10811 * t10978;
    let t45919 = t45877 - 0.11916829983950142223e0_f64 * t44070 - 0.63904876589867916128e-1_f64 * t44084 - 0.63904876589867916128e-1_f64 * t44088 + t45882 + t45885 + t45887 + 0.89376224879626066676e-1_f64 * t45888 - t45892 - t45894 - t45898 - t45900 + t45903 - t45906 - 0.39722766613167140743e-1_f64 * t45908 * t2028 + t45913 + t45915 - 0.17875244975925213335e0_f64 * t44114 - 0.63904876589867916128e-1_f64 * t44117 + 0.1022478025437886658e1_f64 * t44120;
    t45919
}
