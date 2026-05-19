//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 720/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk720<F: Float>(t13176: F, t943: F, t10789: F, t948: F, t2508: F, t10924: F, t2558: F, t9647: F, t1029: F, t3276: F, t3433: F, t954: F) -> (F, F, F, F, F, F, F, F) {
    let t13177 = t943 * t13176;
    let t13179 = t10789 * t948;
    let t13180 = t2508 * t13179;
    let t13182 = t10924 * t2558;
    let t13183 = t9647 * t13182;
    let t13184 = F::cast_from(0.64087718584518535698e-3_f64) * t13183;
    let t13185 = t3276 * t1029;
    let t13187 = F::cast_from(0.53833683610995569986e-1_f64) * t2508 * t13185;
    let t13188 = t954 * t3433;
    (t13177, t13179, t13180, t13182, t13184, t13185, t13187, t13188)
}
