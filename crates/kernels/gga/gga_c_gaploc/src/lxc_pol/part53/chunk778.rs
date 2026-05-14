//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 778/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk778<F: Float>(t40850: F, t40853: F, t2508: F, t2927: F, t3266: F, t3234: F, t8469: F, t2580: F, t2958: F, t9688: F, t13221: F, t7129: F, t2963: F, t3276: F, t3209: F, t8483: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t43208 = 0.1281754371690370714e-2 * t40850;
    let t43209 = 0.64087718584518535698e-3 * t40853;
    let t43212 = 0.76905262301422242837e-2 * t2508 * t3266 * t2927;
    let t43213 = t8469 * t3234;
    let t43216 = 0.15381052460284448567e-1 * t2508 * t2580 * t43213;
    let t43217 = t2958 * t9688;
    let t43220 = 0.15381052460284448567e-1 * t2508 * t2580 * t43217;
    let t43222 = 0.76905262301422242837e-2 * t7129 * t13221;
    let t43237 = 0.53833683610995569986e-1 * t2508 * t3276 * t2963;
    let t43240 = t8483 * t3209;
    (t43208, t43209, t43212, t43213, t43216, t43217, t43220, t43222, t43237, t43240)
}
