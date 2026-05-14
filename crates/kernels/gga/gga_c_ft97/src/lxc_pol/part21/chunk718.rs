//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 718/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk718<F: Float>(t12359: F, t12362: F, t12571: F, t13102: F, t13108: F, t13117: F, t13120: F, t16706: F, t9166: F, t9369: F, t9371: F, t2086: F, t4778: F, t590: F, t91: F, t4753: F, t9252: F) -> (F, F, F) {
    let t17225 = t13102 - t13108 - t9369 - t9371 - t13117 + 4.0 / 9.0 * t12359 - 8.0 / 27.0 * t12362 - t9166 + t13120 - 8.0 / 9.0 * t12571 - 2.0 / 9.0 * t16706;
    let t17235 = t2086 * t4778;
    let t17237 = t91 * t17235 * t590;
    let t17239 = t9252 * t4753;
    (t17225, t17237, t17239)
}
