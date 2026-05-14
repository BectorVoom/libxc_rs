//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 507/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk507<F: Float>(t1587: F, t538: F, t398: F, t1591: F, t1586: F, t1579: F, t3969: F, t1582: F, t3973: F, t1580: F, t1581: F, t3283: F, t1312: F, t3532: F, t539: F, t3278: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t4374 = 1.0 / t1587 / t538;
    let t4375 = t398 * t4374;
    let t4376 = t1591 * t1591;
    let t4377 = t4375 * t4376;
    let t4378 = t1586 * t4377;
    let t4381 = t1579 * t3969;
    let t4384 = t3973 * t1582;
    let t4385 = t1580 * t4384;
    let t4387 = t1581 * t3283;
    let t4388 = t1312 * t4387;
    let t4391 = t539 * t3532;
    let t4392 = t4391 * t3278;
    (t4374, t4376, t4377, t4378, t4381, t4384, t4385, t4387, t4388, t4391, t4392)
}
