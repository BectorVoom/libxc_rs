//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 374/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk374<F: Float>(t2294: F, t637: F, t639: F, t2251: F, t2254: F, t2256: F, t2261: F, t2265: F, t2268: F, t2273: F, t2277: F, t2284: F, t631: F, t184: F, t21: F, t648: F) -> (F, F, F, F, F) {
    let t2296 = t637 * t639 * t2294;
    let t2299 = -t2251 - 2.0 / 9.0 * t2254 - 2.0 / 3.0 * t2256 + t631 * t2261 / 18.0 - 2.0 / 3.0 * t2265 * t2268 - t631 * t2273 / 3.0 + t631 * t2277 / 6.0 - 3.0 / 2.0 * t631 * t2284 + t631 * t2296 / 2.0;
    let t2300 = t2299 * t184;
    let t2301 = t2300 * t21;
    let t2304 = t648 * t648;
    (t2296, t2299, t2300, t2301, t2304)
}
