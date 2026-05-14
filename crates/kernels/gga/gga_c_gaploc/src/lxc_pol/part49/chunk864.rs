//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 864/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk864<F: Float>(t3234: F, t8469: F, t2508: F, t2580: F, t2958: F, t9688: F, t13221: F, t7129: F, t2558: F, t33232: F, t9647: F, t13188: F, t13203: F, t2963: F, t3276: F, t3209: F, t8483: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t43213 = t8469 * t3234;
    let t43216 = 0.15381052460284448567e-1 * t2508 * t2580 * t43213;
    let t43217 = t2958 * t9688;
    let t43220 = 0.15381052460284448567e-1 * t2508 * t2580 * t43217;
    let t43222 = 0.76905262301422242837e-2 * t7129 * t13221;
    let t43224 = t9647 * t33232 * t2558;
    let t43231 = t7129 * t13188;
    let t43233 = t7129 * t13203;
    let t43237 = 0.53833683610995569986e-1 * t2508 * t3276 * t2963;
    let t43240 = t8483 * t3209;
    (t43213, t43216, t43217, t43220, t43222, t43224, t43231, t43233, t43237, t43240)
}
