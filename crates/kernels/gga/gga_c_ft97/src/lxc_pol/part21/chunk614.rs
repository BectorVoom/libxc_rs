//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 614/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk614<F: Float>(t12310: F, t12327: F, t12356: F, t12365: F, t157: F, t526: F) -> (F, F, F, F, F) {
    let t13102 = 4.0 / 27.0 * t12310;
    let t13108 = 2.0 / 9.0 * t12327;
    let t13117 = 4.0 / 3.0 * t12356;
    let t13120 = 2.0 / 3.0 * t12365;
    let t13140 = t526 * t157;
    (t13102, t13108, t13117, t13120, t13140)
}
