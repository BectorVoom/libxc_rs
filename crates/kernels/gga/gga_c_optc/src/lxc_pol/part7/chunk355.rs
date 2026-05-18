//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 355/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk355<F: Float>(t1122: F, t449: F, t438: F, t894: F, t1129: F, t466: F, t155: F, t463: F, t1132: F) -> (F, F, F, F) {
    let t1171 = t449 * t1122;
    let t1172 = t1171 * t438;
    let t1173 = t894 * t1172;
    let t1177 = F::new(0.50380704458364197288e-2) * t466 * t1129;
    let t1178 = t155 * t463;
    let t1179 = t1178 * t1132;
    (t1172, t1173, t1177, t1179)
}
