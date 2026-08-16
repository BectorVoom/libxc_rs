//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1366/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1366(t26960: f64, t28096: f64, t2850: f64, t3515: f64, t93006: f64, t93008: f64, t93010: f64, t93012: f64, t93014: f64, t93016: f64, t96061: f64, t96068: f64, t96071: f64, t96116: f64, t96910: f64) -> f64 {
    let t97232 = 0.23214722222222222222e-2_f64 * t93006 + 0.61905925925925925926e-2_f64 * t93008 - 0.23214722222222222222e-2_f64 * t93010 + 0.15476481481481481481e-2_f64 * t93012 - 0.61905925925925925925e-2_f64 * t96061 - 0.41270617283950617284e-2_f64 * t96068 + 0.92858888888888888886e-2_f64 * t96071 - 0.11326774691358024691e-2_f64 * t93014 - 0.41188271604938271606e-3_f64 * t93016 + 0.13901041666666666667e-2_f64 * t26960 * t96910 - 0.23168402777777777778e-3_f64 * t26960 * t3515 * t28096 * t2850 - 0.15476481481481481481e-2_f64 * t96116;
    t97232
}
