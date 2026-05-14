//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1253/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1253<F: Float>(t2354: F, t24526: F, t446: F, t4635: F, t2371: F, t30859: F, t193: F, t713: F, t89: F, t24191: F, t5053: F, t4934: F, t96824: F, t108278: F, t108284: F, t124054: F, t124058: F, t124061: F, t124065: F, t124069: F) -> (F, F, F, F, F, F) {
    let t124074 = t446 * t2354 * t24526 * t4635;
    let t124076 = t2371 * t30859;
    let t124079 = t89 * t193 * t124076 * t713;
    let t124083 = t89 * t193 * t24191 * t5053;
    let t124087 = t89 * t193 * t96824 * t4934;
    let t124089 = -t124054 / 27.0 + t124058 / 9.0 - 4.0 / 27.0 * t124061 + t124065 / 2.0 + t124069 + 8.0 / 27.0 * t108278 + 4.0 / 81.0 * t108284 + t124074 / 9.0 + 2.0 / 3.0 * t124079 + 2.0 / 3.0 * t124083 - 2.0 * t124087;
    (t124074, t124076, t124079, t124083, t124087, t124089)
}
