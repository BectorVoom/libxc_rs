//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1000/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1000<F: Float>(t370: F, t480: F, t499: F, t8216: F, t1780: F, t1852: F, t1786: F, t3238: F, t463: F, t8418: F, t10: F, t16: F, t378: F) -> (F, F, F, F, F, F) {
    let t47120 = t370 * t480;
    let t47231 = t8216 * t499;
    let t47399 = t1780 * t1852;
    let t47443 = t1786 * t3238;
    let t47548 = t463 * t8418;
    let t47659 = t10 * t16 * t378;
    (t47120, t47231, t47399, t47443, t47548, t47659)
}
