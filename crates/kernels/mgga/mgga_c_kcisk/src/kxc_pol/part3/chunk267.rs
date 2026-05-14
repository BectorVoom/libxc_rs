//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 267/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk267<F: Float>(t385: F, t1280: F, t1287: F, t340: F, t379: F, t382: F, sigma0: F) -> (F,) {
    let t386 = t385 < -0.66725e-1;
    let t1292 = piecewise3(t386, 0.0, 10.0 / 9.0 * t340 * t1280 * t382 - 10.0 / 27.0 * t340 * t379 * t1287);
    let t1293 = t1292 * sigma0;
    (t1293,)
}
