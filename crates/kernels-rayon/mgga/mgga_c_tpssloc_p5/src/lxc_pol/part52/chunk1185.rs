//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1185/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1185(t31046: f64, t31050: f64, t31052: f64, t31055: f64, t31057: f64, t31060: f64, t31065: f64, t31067: f64, t31070: f64, t31072: f64, t31077: f64, t650: f64, t6517: f64, t7271: f64, t8682: f64) -> f64 {
    let t31849 = -t650 * t8682 - 2.0_f64 * t6517 * t7271 + t31046 + t31050 - 2.0_f64 * t31052 - t31055 - t31057 - t31060 - 2.0_f64 * t31065 - 2.0_f64 * t31067 - 2.0_f64 * t31070 - 2.0_f64 * t31072 - t31077;
    t31849
}
