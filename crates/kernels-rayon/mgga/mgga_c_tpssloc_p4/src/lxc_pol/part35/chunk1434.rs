//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1434/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1434(t104907: f64, t104968: f64, t106836: f64, t106855: f64, t2110: f64, t27332: f64, t27961: f64, t27972: f64, t27976: f64, t27982: f64, t29475: f64, t29478: f64, t29481: f64, t7432: f64, t7435: f64, t7975: f64, t7978: f64, t85501: f64, t96120: f64) -> f64 {
    let t109025 = 5.0_f64 / 2.0_f64 * t104968 * t7432 + 5.0_f64 * t27332 * t27972 + 5.0_f64 / 2.0_f64 * t27332 * t27976 + t106855 * t2110 / 3.0_f64 + t27982 * t7975 + t27982 * t7978 + t7435 * t29475 + 2.0_f64 * t7435 * t29478 + t7435 * t29481 - 15.0_f64 * t96120 * t27961 + 35.0_f64 * t85501 * t106836 - 5.0_f64 * t104907 * t7432;
    t109025
}
