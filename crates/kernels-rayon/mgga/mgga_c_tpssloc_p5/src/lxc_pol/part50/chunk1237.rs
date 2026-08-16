//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1237/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1237(t26179: f64, t8323: f64, t31069: f64, t7458: f64, t120019: f64, t120020: f64, t120022: f64, t120027: f64, t120029: f64, t120040: f64, t120044: f64, t120045: f64, t120047: f64, t120049: f64, t120051: f64, t120053: f64, t1458: f64, t2314: f64, t24983: f64, t30989: f64, t32656: f64, t4034: f64, t4072: f64, t6517: f64, t652: f64, t8439: f64) -> f64 {
    let t120055 = t26179 * t8323;
    let t120057 = t7458 * t31069;
    let t120059 = -2.0_f64 * t1458 * t30989 * t652 - 2.0_f64 * t4072 * t652 * t8439 - 2.0_f64 * t2314 * t32656 - 4.0_f64 * t24983 * t6517 - 2.0_f64 * t32656 * t4034 - t120019 - 4.0_f64 * t120020 - 4.0_f64 * t120022 - 4.0_f64 * t120027 - 4.0_f64 * t120029 - 4.0_f64 * t120040 + t120044 - 4.0_f64 * t120045 - 4.0_f64 * t120047 - 4.0_f64 * t120049 - 4.0_f64 * t120051 - 4.0_f64 * t120053 - 4.0_f64 * t120055 - 4.0_f64 * t120057;
    t120059
}
