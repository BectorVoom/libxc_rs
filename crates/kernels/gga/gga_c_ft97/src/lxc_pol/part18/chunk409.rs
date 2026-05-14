//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 409/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk409<F: Float>(t3255: F, t488: F, t83: F, t1882: F, t955: F, t1825: F, t979: F, t432: F, t942: F) -> (F, F, F, F) {
    let t3256 = t488 * t3255;
    let t3257 = t83 * t3256;
    let t3260 = t1882 * t955;
    let t3262 = t1825 * t979;
    let t3263 = t83 * t3262;
    let t3266 = t942 * t432;
    (t3257, t3260, t3263, t3266)
}
