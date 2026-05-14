//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 917/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk917<F: Float>(t41460: F, t41463: F, t44130: F, t44134: F, t44138: F, t44142: F, t44144: F, t44147: F, t44148: F, t44149: F, t44150: F, t44154: F, t44155: F, t47531: F, t47535: F, t47537: F, t47540: F, t47544: F, t47549: F, t47552: F) -> (F,) {
    let t51183 = -0.13803453343411469884e2 * t47531 - 0.13803453343411469884e2 * t47535 + 0.23005755572352449806e2 * t47537 + 0.23005755572352449806e2 * t47540 - 0.92023022289409799224e1 * t47544 - 0.89376224879626066674e-1 * t44130 + t44134 + 0.85801175884441024008e1 * t47549 - 0.42900587942220512004e1 * t47552 + t44138 + t44142 + t44144 + t44147 - t44148 + t44149 + t44150 + 0.35750489951850426669e0 * t41460 - 0.17875244975925213334e0 * t41463 - t44154 + t44155;
    (t51183,)
}
