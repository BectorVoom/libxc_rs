//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 742/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk742<F: Float>(t24237: F, t6005: F, t263: F, t6061: F, t684: F, t2354: F, t1424: F, t771: F, t1403: F, t1454: F, t24179: F, t24184: F, t24188: F, t24193: F, t24197: F, t24201: F, t24204: F, t24208: F, t24213: F, t24217: F, t24221: F, t24224: F, t24228: F, t24234: F, t2465: F, t6002: F, t6192: F, t719: F) -> (F, F, F, F, F, F) {
    let t24238 = t24237 * t6005;
    let t24240 = t6061 * t263;
    let t24241 = t24240 * t684;
    let t24242 = t2354 * t24241;
    let t24245 = t1424 * t771;
    let t24247 = t2354 * t24245 * t684;
    let t24250 = 2.0 / 9.0 * t24179 + t1403 * t24184 - t1403 * t24188 / 3.0 - 2.0 / 3.0 * t1403 * t24193 - t6002 * t24197 / 18.0 - t6002 * t24201 / 27.0 - t24204 * t6005 / 9.0 + t1403 * t24208 / 3.0 + t24213 - 2.0 * t719 * t6192 + t1403 * t24217 / 6.0 - t24221 / 9.0 - t24224 / 9.0 - t2465 * t1454 + t6002 * t24228 / 9.0 + 2.0 / 9.0 * t6002 * t24234 + t24238 / 27.0 - t6002 * t24242 / 9.0 - t6002 * t24247 / 9.0;
    (t24238, t24240, t24242, t24245, t24247, t24250)
}
