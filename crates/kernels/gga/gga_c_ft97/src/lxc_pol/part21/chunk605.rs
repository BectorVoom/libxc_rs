//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 605/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk605<F: Float>(t1018: F, t1636: F, t89: F, t1026: F, t8232: F, t1882: F, t3463: F, t3480: F, t3485: F, t1045: F, t2178: F) -> (F, F, F, F, F, F) {
    let t12571 = t89 * t1636 * t1018;
    let t12617 = t8232 * t1026;
    let t12620 = 2.0 / 27.0 * t1882 * t3463;
    let t12642 = 2.0 / 9.0 * t1882 * t3480;
    let t12644 = 4.0 / 9.0 * t1882 * t3485;
    let t12664 = t1045 * t2178;
    (t12571, t12617, t12620, t12642, t12644, t12664)
}
