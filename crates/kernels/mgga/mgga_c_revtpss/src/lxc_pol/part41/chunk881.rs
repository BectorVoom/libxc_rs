//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 881/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk881<F: Float>(t1150: F, t6470: F, t1131: F, t3435: F, t6438: F, t3433: F, t3439: F, t5044: F, t6423: F, t6427: F, t6431: F, t1744: F, t1169: F, t3459: F, t3466: F, t5093: F, t6443: F, t6450: F, t6456: F, t6458: F, t6462: F, t6465: F, t6468: F) -> (F, F, F, F, F, F, F, F) {
    let t6471 = t6470 * t1150;
    let t6473 = 1.0 * t1131 * t6471;
    let t6474 = t6438 * t3435;
    let t6476 = 0.16081979498692535067e2 * t3433 * t6474;
    let t6481 = t3439 - 0.11415555555555555555e-1 * t5044 - 0.11415555555555555555e-1 * t6423 + 0.34246666666666666666e-1 * t6427 + 0.17123333333333333333e-1 * t6431;
    let t6486 = t1744 * t1744;
    let t6487 = t6486 * t1169;
    let t6502 = -0.17648625e1 * t6443 + 0.3529725e1 * t6450 + t3459 - 0.34431666666666666666e0 * t5044 - 0.34431666666666666667e0 * t6423 + 0.103295e1 * t6427 + 0.516475e0 * t6431 + 0.31558125e0 * t6456 + 0.6311625e0 * t6458 + t3466 - 0.13892666666666666667e0 * t5093 - 0.34731666666666666667e-1 * t6462 + 0.20839e0 * t6465 + 0.104195e0 * t6468;
    (t6471, t6473, t6474, t6476, t6481, t6486, t6487, t6502)
}
