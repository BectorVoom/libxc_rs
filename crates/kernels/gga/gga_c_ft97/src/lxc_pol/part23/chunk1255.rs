//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1255/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1255<F: Float>(t10157: F, t31029: F, t446: F, t713: F, t1882: F, t31001: F, t31005: F, t24437: F, t24546: F, t2574: F, t30970: F, t31044: F, t681: F, t89: F, t108291: F, t110151: F, t124093: F, t124096: F, t124101: F, t124106: F, t97352: F) -> (F, F, F, F, F, F) {
    let t124110 = t446 * t10157 * t31029 * t713;
    let t124112 = t1882 * t31001;
    let t124114 = t1882 * t31005;
    let t124118 = t24437 * t2574 * t24546 * t30970;
    let t124121 = t89 * t681 * t31044;
    let t124123 = 16.0 / 27.0 * t108291 - t124093 / 18.0 + t124096 / 27.0 + 2.0 / 3.0 * t124101 - t124106 / 8.0 - 2.0 * t124110 - t124112 / 27.0 - 2.0 / 81.0 * t124114 + t97352 - t124118 / 3.0 + 2.0 / 3.0 * t124121 - t110151;
    (t124110, t124112, t124114, t124118, t124121, t124123)
}
