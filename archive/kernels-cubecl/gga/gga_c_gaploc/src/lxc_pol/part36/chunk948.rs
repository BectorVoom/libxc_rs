//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 948/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk948<F: Float>(t3234: F, t8469: F, t2508: F, t2580: F, t2958: F, t9688: F, t13221: F, t7129: F, t2558: F, t33232: F, t9647: F, t13188: F) -> (F, F, F, F, F, F, F) {
    let t43213 = t8469 * t3234;
    let t43216 = F::cast_from(0.15381052460284448567e-1_f64) * t2508 * t2580 * t43213;
    let t43217 = t2958 * t9688;
    let t43220 = F::cast_from(0.15381052460284448567e-1_f64) * t2508 * t2580 * t43217;
    let t43222 = F::cast_from(0.76905262301422242837e-2_f64) * t7129 * t13221;
    let t43224 = t9647 * t33232 * t2558;
    let t43231 = t7129 * t13188;
    (t43213, t43216, t43217, t43220, t43222, t43224, t43231)
}
