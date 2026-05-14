//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 146/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk146<F: Float>(t498: F, t499: F, t493: F, t489: F, t475: F, t303: F) -> (F, F, F, F, F, F) {
    let t500 = t498 * t499;
    let t501 = t493 * t500;
    let t503 = 1.0 + t489 / 16.0 - t501 / 256.0;
    let t504 = 1.0 / t503;
    let t505 = t475 * t504;
    let t507 = 1.0 + 0.5137e-1 * t303;
    (t500, t501, t503, t504, t505, t507)
}
