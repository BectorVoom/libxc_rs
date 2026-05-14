//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 646/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk646<F: Float>(t13101: F, t738: F, t13096: F, t169: F, t299: F, t706: F, t2558: F, t3464: F, t943: F, t10789: F, t948: F, t2508: F, t10924: F, t9647: F, t1029: F, t3276: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t13168 = t738 * t13101;
    let t13172 = t13096 * t169 * t299;
    let t13173 = t706 * t13172;
    let t13176 = t3464 * t2558;
    let t13177 = t943 * t13176;
    let t13179 = t10789 * t948;
    let t13180 = t2508 * t13179;
    let t13182 = t10924 * t2558;
    let t13183 = t9647 * t13182;
    let t13184 = 0.64087718584518535698e-3 * t13183;
    let t13185 = t3276 * t1029;
    (t13168, t13172, t13173, t13176, t13177, t13179, t13180, t13182, t13184, t13185)
}
