//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2572/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2572(t43816: f64, t51349: f64, t51354: f64, t63361: f64, t63382: f64, t63384: f64, t63398: f64, t63400: f64, t64074: f64, t64076: f64, t64087: f64, t64089: f64) -> f64 {
    let t71989 = -0.5356037037037037037e0_f64 * t43816 + t51349 - t51354 + 0.13772666666666666667e1_f64 * t63361 + 0.68863333333333333332e0_f64 * t63382 + 0.20658999999999999999e1_f64 * t63384 - 0.20659e1_f64 * t63398 - 0.309885e1_f64 * t63400 + 0.13892666666666666667e0_f64 * t64074 + 0.41678000000000000001e0_f64 * t64076 - 0.83356000000000000002e0_f64 * t64087 - 0.125034e1_f64 * t64089;
    t71989
}
