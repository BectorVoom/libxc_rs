//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 993/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk993<F: Float>(t26183: F, t26228: F, t26275: F, t26315: F, t26363: F, t26407: F, t26457: F, t26490: F, t1286: F, t1337: F, t22915: F, t23090: F, t26056: F, t26059: F, t26062: F, t26114: F, t26119: F, t26125: F, t26130: F, t2976: F, t3109: F, t438: F, t5495: F, t5501: F, t5748: F, t6423: F, t6562: F, t88: F, t948: F) -> (F, F) {
    let t26493 = t26183 + t26228 + t26275 + t26315 + t26363 + t26407 + t26457 + t26490;
    let t26496 = -t2976 * t1337 - 2.0 * t26056 + t22915 / 54.0 - 2.0 * t26059 - 2.0 * t26062 - t948 * t5748 - 2.0 * t26114 - t3109 * t1337 - t5501 * t26119 / 18.0 - t5495 * t6423 / 3.0 - t1286 * t26125 / 3.0 - t1286 * t26130 / 3.0 - t23090 / 18.0 - t88 * t26493 - t438 * t6562;
    (t26493, t26496)
}
