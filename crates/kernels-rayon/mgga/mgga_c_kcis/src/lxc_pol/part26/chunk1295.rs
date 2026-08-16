//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1295/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1295(t21007: f64, t5661: f64, t98530: f64, t102137: f64, t102142: f64, t102151: f64, t102155: f64, t20882: f64, t27583: f64, t28701: f64, t28758: f64, t6159: f64, t7968: f64, t7978: f64, t8213: f64, t95001: f64, t99233: f64, t99282: f64, t99293: f64, t99565: f64) -> (f64, f64) {
    let t102158 = t5661 * t98530 * t21007;
    let t102164 = 0.30918233506944444445e-4_f64 * t99565 * t28701 - 0.46336805555555555556e-3_f64 * t27583 * t102137 - t99282 + 0.25742669753086419753e-4_f64 * t95001 + 0.34752604166666666667e-3_f64 * t7978 * t102142 - 0.24734586805555555556e-3_f64 * t99233 * t8213 + 0.46377350260416666667e-4_f64 * t7968 * t102142 + 0.11584201388888888889e-3_f64 * t27583 * t102151 - 0.30952962962962962962e-2_f64 * t102155 + t99293 - 0.38691203703703703703e-2_f64 * t102158 + 0.23168402777777777778e-3_f64 * t27583 * t6159 * t28758 * t20882;
    (t102158, t102164)
}
