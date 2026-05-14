//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 836/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk836<F: Float>(t25378: F, t296: F, t1501: F, t2682: F, t2862: F, t871: F, t1901: F, t193: F, t25287: F, t25291: F, t25295: F, t25298: F, t25301: F, t25305: F, t25309: F, t25312: F, t25315: F, t25317: F, t25320: F, t25324: F, t25362: F, t25366: F, t25370: F, t25374: F, t446: F, t89: F) -> (F, F, F, F) {
    let t25379 = t296 * t25378;
    let t25382 = t1501 * t2682;
    let t25384 = t2862 * t871 * t25382;
    let t25387 = 2.0 / 9.0 * t1901 * t25287 - t446 * t25291 / 3.0 - 2.0 / 3.0 * t446 * t25295 + 2.0 / 9.0 * t25298 + 2.0 / 3.0 * t446 * t25301 - t446 * t25305 / 9.0 - 2.0 / 27.0 * t446 * t25309 + 2.0 / 27.0 * t25312 - 2.0 / 9.0 * t25315 + 2.0 / 9.0 * t25317 - t446 * t25320 / 3.0 - 2.0 / 3.0 * t446 * t25324 + t89 * t193 * t25362 / 3.0 + 2.0 / 9.0 * t25366 + 2.0 / 9.0 * t1901 * t25370 - 2.0 / 9.0 * t1901 * t25374 - 2.0 * t446 * t25379 - 2.0 / 3.0 * t446 * t25384;
    (t25379, t25382, t25384, t25387)
}
