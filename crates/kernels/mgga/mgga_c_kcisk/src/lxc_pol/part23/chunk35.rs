//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 35/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk35<F: Float>(t24: F, t81: F, t80: F, t71: F, t74: F, t77: F, t45: F, t67: F, t73: F, t10: F, t64: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t82 = t81 * t24;
    let t83 = t80 * t82;
    let t85 = 0.379785e1 * t74 + 0.8969e0 * t71 + 0.204775e0 * t77 + 0.123235e0 * t83;
    let t88 = 1.0 + 0.16081824322151104822e2 / t85;
    let t89 = f64::ln(t88);
    let t93 = 1.0 + 0.278125e-1 * t71;
    let t98 = 0.51785e1 * t74 + 0.905775e0 * t71 + 0.1100325e0 * t77 + 0.1241775e0 * t83;
    let t101 = 1.0 + 0.29608574643216675549e2 / t98;
    let t102 = f64::ln(t101);
    let t107 = t67 * (-0.62182e-1 * t73 * t89 + 0.19751789702565206229e-1 * t45 * t93 * t102);
    let t110 = 10.0 / 9.0 * t64 * t107 * t10;
    let t111 = t110 < -0.66725e-1;
    let t113 = piecewise3(t111, 0.0, 0.66725e-1 + t110);
    (t83, t85, t88, t89, t93, t98, t101, t102, t107, t113)
}
