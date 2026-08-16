//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 817/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk817(t24049: f64, t24050: f64, t24058: f64, t24060: f64, t24061: f64, t26272: f64, t26295: f64, t28085: f64, t28089: f64, t28091: f64, t28093: f64, t28095: f64, t28097: f64, t28102: f64, t28104: f64) -> f64 {
    let t29285 = 0.80745512188280781706e-3_f64 * t26272 + t28085 / 384.0_f64 - t24049 + t24050 + 0.56521858531796547194e-2_f64 * t26295 + t28089 / 768.0_f64 - t28091 / 768.0_f64 + 5.0_f64 / 192.0_f64 * t28093 - t28095 / 192.0_f64 - t28097 / 96.0_f64 + 0.48447307312968469024e-2_f64 * t28102 + t24058 + t24060 + t24061 + t28104 / 96.0_f64;
    t29285
}
