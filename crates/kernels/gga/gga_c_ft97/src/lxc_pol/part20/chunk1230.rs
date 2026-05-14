//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1230/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1230<F: Float>(t1486: F, t28800: F, t681: F, t12001: F, t28789: F, t2399: F, t7075: F, t1934: F, t2665: F, t28533: F, t446: F, t2347: F, t7021: F, t10409: F, t2349: F, t113248: F, t113250: F, t113252: F, t113254: F, t113257: F, t113261: F, t113265: F) -> (F, F, F, F, F, F) {
    let t113268 = t1486 * t681 * t28800;
    let t113269 = t113268 / 3.0;
    let t113270 = t12001 * t28789;
    let t113273 = t1486 * t2399 * t7075;
    let t113274 = 2.0 / 9.0 * t113273;
    let t113277 = t446 * t2665 * t28533 * t1934;
    let t113279 = t7021 * t2347;
    let t113282 = t446 * t10409 * t113279 * t2349;
    let t113284 = -t113248 + t113250 - t113252 + t113254 - 4.0 / 3.0 * t113257 - 12.0 * t113261 - t113265 / 2.0 + t113269 + 22.0 / 9.0 * t113270 - t113274 + t113277 / 3.0 + 2.0 / 9.0 * t113282;
    (t113268, t113270, t113273, t113277, t113282, t113284)
}
