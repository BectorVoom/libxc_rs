//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 864/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk864<F: Float>(t25596: F, t83: F, t3271: F, t452: F, t5710: F, t1901: F, t23183: F, t23199: F, t23227: F, t23229: F, t26364: F, t26368: F, t26375: F, t26379: F, t26383: F, t26387: F, t26392: F, t26395: F, t446: F) -> (F, F, F) {
    let t26398 = t83 * t25596;
    let t26402 = t452 * t5710 * t3271;
    let t26407 = t1901 * t26364 / 9.0 - 2.0 / 3.0 * t1901 * t26368 - 2.0 * t1901 * t26375 - 2.0 / 3.0 * t1901 * t26379 - 2.0 / 3.0 * t1901 * t26383 - t23183 - t23199 / 9.0 - t446 * t26387 / 3.0 + t446 * t26392 / 3.0 + 2.0 / 3.0 * t446 * t26395 + 2.0 / 3.0 * t446 * t26398 + t446 * t26402 / 3.0 - 2.0 / 9.0 * t23227 - t23229 / 9.0;
    (t26398, t26402, t26407)
}
