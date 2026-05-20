//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 900/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk900<F: Float>(t300: F, t6513: F, t1179: F, t1160: F, t6481: F, t3479: F, t6502: F, t1130: F, t6433: F, t3435: F, t6470: F, t3523: F, t6534: F) -> (F, F, F, F, F, F, F) {
    let t20400 = t300 * t6513;
    let t20526 = t6513 * t1179;
    let t20542 = t6481 * t1160;
    let t20618 = t6502 * t3479;
    let t20629 = t6433 * t1130;
    let t20644 = t6470 * t3435;
    let t20671 = t6534 * t3523;
    (t20400, t20526, t20542, t20618, t20629, t20644, t20671)
}
