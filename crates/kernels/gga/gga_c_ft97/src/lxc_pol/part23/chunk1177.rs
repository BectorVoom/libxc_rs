//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1177/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1177<F: Float>(t7051: F, t8232: F, t1882: F, t29274: F, t29359: F, t29329: F, t29129: F, t310: F, t29346: F, t29366: F, t29151: F, t8392: F, t29113: F, t29247: F, t7047: F, t29313: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t113981 = t8232 * t7051;
    let t114001 = 2.0 / 27.0 * t1882 * t29274;
    let t114055 = 4.0 / 9.0 * t1882 * t29359;
    let t114062 = 4.0 / 9.0 * t1882 * t29329;
    let t114104 = t310 * t29129;
    let t114142 = 2.0 / 9.0 * t1882 * t29346;
    let t114162 = 4.0 / 9.0 * t1882 * t29366;
    let t114164 = 2.0 / 27.0 * t8392 * t29151;
    let t114194 = 2.0 / 27.0 * t8392 * t29113;
    let t114196 = 2.0 / 9.0 * t1882 * t29247;
    let t114197 = t8232 * t7047;
    let t114211 = 4.0 / 9.0 * t1882 * t29313;
    (t113981, t114001, t114055, t114062, t114104, t114142, t114162, t114164, t114194, t114196, t114197, t114211)
}
