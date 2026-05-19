//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1008/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1008<F: Float>(t11004: F, t2508: F, t7226: F, t7291: F, t40902: F, t10789: F, t7667: F, t13188: F, t7137: F, t13191: F, t7129: F, t24660: F, t3251: F) -> (F, F, F, F, F, F) {
    let t43298 = t2508 * t7226 * t11004 * t7291;
    let t43300 = F::cast_from(0.64087718584518535698e-3_f64) * t40902;
    let t43302 = t2508 * t10789 * t7667;
    let t43304 = t7137 * t13188;
    let t43312 = F::cast_from(0.92286314761706691403e-1_f64) * t7129 * t13191;
    let t43315 = F::cast_from(0.92286314761706691403e-1_f64) * t2508 * t24660 * t3251;
    (t43298, t43300, t43302, t43304, t43312, t43315)
}
