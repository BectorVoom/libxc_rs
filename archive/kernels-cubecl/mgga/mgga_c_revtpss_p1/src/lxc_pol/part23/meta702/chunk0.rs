//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2452/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2452<F: Float>(t9860: F, t9866: F, t9863: F, t3869: F, t39532: F, t9575: F, t39538: F, t39427: F, t39535: F, t4038: F, t9372: F, t1317: F, t9428: F) -> (F, F, F, F, F, F, F, F, F) {
    let t47125 = t9860 * t9866;
    let t47127 = t9860 * t9863;
    let t47131 = F::cast_from(0.21687162600603479684e-1_f64) * t3869 * t39532;
    let t47135 = t9860 * t9575;
    let t47138 = F::cast_from(0.43374325201206959368e-1_f64) * t3869 * t39538;
    let t47140 = F::cast_from(0.12842595503380418954e1_f64) * t3869 * t39427;
    let t47142 = F::cast_from(0.38025319932552508021e2_f64) * t3869 * t39535;
    let t47147 = t4038 * t9372;
    let t47149 = t1317 * t9428;
    (t47125, t47127, t47131, t47135, t47138, t47140, t47142, t47147, t47149)
}
