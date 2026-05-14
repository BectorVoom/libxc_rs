//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1326/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1326<F: Float>(t3: F, t31204: F, t2198: F, t2327: F, t116: F, t8320: F, t670: F, t2371: F, t8342: F, t117: F, t31157: F, t1459: F, t1461: F, t2207: F, t2209: F, t4158: F, t4162: F, t4165: F, t572: F, t573: F, t8336: F, t8343: F, t8346: F) -> (F, F, F, F, F, F, F, F) {
    let t31205 = t3 * t31204;
    let t31217 = param_d * t31204;
    let t31231 = t2327 * t2198;
    let t31234 = t116 * t8320;
    let t31235 = t31234 * t670;
    let t31238 = t8342 * t2371;
    let t31241 = t117 * t31157;
    let t31244 = 12.0 * t1459 * t8343 + 6.0 * t1459 * t8346 + 6.0 * t1461 * t8336 + 6.0 * t2207 * t4162 + 3.0 * t2207 * t4165 + 3.0 * t2209 * t4158 + t31217 * t573 + 6.0 * t31231 * t572 + 12.0 * t31235 * t572 + 6.0 * t31238 * t572 + 3.0 * t31241 * t572;
    (t31205, t31217, t31231, t31234, t31235, t31238, t31241, t31244)
}
