//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 54/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk54<F: Float>(t44: F, t51: F, t132: F, t133: F, t129: F, t130: F, t98: F, t99: F, t101: F, t108: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t45 = t44 <= zeta_threshold;
    let t52 = t51 <= zeta_threshold;
    let t134 = t132 * t133;
    let t135 = t129 * t130 * t134;
    let t138 = t98 * zeta_threshold;
    let t139 = t99 * t44;
    let t140 = piecewise3(t45, t138, t139);
    let t141 = t101 * t51;
    let t142 = piecewise3(t52, t138, t141);
    let t144 = t140 / 2.0 + t142 / 2.0;
    let t146 = t108 / t144;
    (t134, t135, t139, t141, t144, t146)
}
