//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 13/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk13<F: Float>(t40: F, t37: F, t11: F, t14: F, t17: F, t25: F, t2: F, t3: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t43 = 1.0 / (2.0 * t40 - 2.0);
    let t44 = (2.0 * t37 - 2.0) * t43;
    let t46 = 1.0 + 0.278125e-1 * t11;
    let t51 = 0.51785e1 * t14 + 0.905775e0 * t11 + 0.1100325e0 * t17 + 0.1241775e0 * t25;
    let t54 = 1.0 + 0.29608574643216675549e2 / t51;
    let t55 = f64::ln(t54);
    let t56 = t46 * t55;
    let t58 = 0.19751789702565206229e-1 * t44 * t56;
    let t59 = t3 * t2;
    let t60 = 1.0 / t59;
    (t43, t44, t46, t51, t54, t55, t56, t58, t59, t60)
}
