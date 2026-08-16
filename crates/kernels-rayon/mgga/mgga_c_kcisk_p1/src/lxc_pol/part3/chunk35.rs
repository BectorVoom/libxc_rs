//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 35/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk35(t24: f64, t81: f64, t80: f64, t71: f64, t74: f64, t77: f64, t45: f64, t67: f64, t73: f64, t10: f64, t64: f64, t41: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t82 = t81 * t24;
    let t83 = t80 * t82;
    let t85 = 0.379785e1_f64 * t74 + 0.8969e0_f64 * t71 + 0.204775e0_f64 * t77 + 0.123235e0_f64 * t83;
    let t88 = 1.0_f64 + 0.16081824322151104822e2_f64 / t85;
    let t89 = f64::ln(t88);
    let t93 = 1.0_f64 + 0.278125e-1_f64 * t71;
    let t98 = 0.51785e1_f64 * t74 + 0.905775e0_f64 * t71 + 0.1100325e0_f64 * t77 + 0.1241775e0_f64 * t83;
    let t101 = 1.0_f64 + 0.29608574643216675549e2_f64 / t98;
    let t102 = f64::ln(t101);
    let t107 = t67 * (-0.62182e-1_f64 * t73 * t89 + 0.19751789702565206229e-1_f64 * t45 * t93 * t102);
    let t110 = 10.0_f64 / 9.0_f64 * t64 * t107 * t10;
    let t111 = t110 < -0.66725e-1_f64;
    let t113 = piecewise3(t111, 0.0_f64, 0.66725e-1_f64 + t110);
    let t114 = t113 * t41;
    (t83, t85, t88, t89, t93, t98, t101, t102, t107, t114, t110)
}
