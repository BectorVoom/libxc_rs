//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1901/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1901(t10784: f64, t10785: f64, t21120: f64, t21124: f64, t21128: f64, t21132: f64, t21136: f64, t21140: f64, t21142: f64, t21144: f64, t21147: f64, t21150: f64, t21153: f64, t21156: f64) -> f64 {
    let t21158 = 0.20839e0_f64 * t21120 - 0.103295e1_f64 * t21124 + 0.309885e1_f64 * t21128 - 0.46308888888888888889e-1_f64 * t21132 - 0.104195e0_f64 * t21136 - 0.62517e0_f64 * t21140 - 0.52945875e1_f64 * t21142 + 0.94674375e0_f64 * t21144 - t10784 - t10785 - 0.57386111111111111112e0_f64 * t21147 + 0.20659e1_f64 * t21150 - 0.309885e1_f64 * t21153 - 0.516475e0_f64 * t21156;
    t21158
}
