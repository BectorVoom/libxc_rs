//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2351/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2351<F: Float>(t11152: F, t76: F, t41: F, t42: F, t53: F, t54: F, t9576: F, t2405: F, t2420: F, t702: F) -> (F, F, F, F, F, F) {
    let t39114 = F::cast_from(1.0_f64) / t76 / t11152;
    let t39157 = t41 * t41;
    let t39159 = F::cast_from(1.0_f64) / t42 / t39157;
    let t39166 = t53 * t53;
    let t39168 = F::cast_from(1.0_f64) / t54 / t39166;
    let t39210 = F::cast_from(20944.0_f64) / F::cast_from(81.0_f64) * t9576;
    let t39246 = t2405 * t2405;
    let t39249 = F::cast_from(6.0_f64) * t2420 * t39246 * t702;
    (t39114, t39159, t39168, t39210, t39246, t39249)
}
