//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 428/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk428<F: Float>(t3255: F, t488: F, t83: F, t1882: F, t955: F, t1825: F, t979: F, t432: F, t942: F, t110: F, t1871: F, t492: F, t452: F, t447: F, t499: F, t925: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3256 = t488 * t3255;
    let t3257 = t83 * t3256;
    let t3260 = t1882 * t955;
    let t3262 = t1825 * t979;
    let t3263 = t83 * t3262;
    let t3266 = t942 * t432;
    let t3268 = t1871 * t110 * t3266;
    let t3271 = t942 * t492;
    let t3273 = t452 * t488 * t3271;
    let t3277 = t447 * t499 * t925;
    (t3256, t3257, t3260, t3262, t3263, t3266, t3268, t3271, t3273, t3277)
}
