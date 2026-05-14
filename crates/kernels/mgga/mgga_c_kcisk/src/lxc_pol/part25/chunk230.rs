//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 230/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk230<F: Float>(t264: F, t67: F, t852: F, t10: F, t142: F, t260: F, t261: F, t116: F) -> (F, F, F) {
    let t265 = t264 < -0.66725e-1;
    let t1102 = t67 * t852;
    let t1110 = piecewise3(t265, 0.0, 10.0 / 9.0 * t260 * t1102 * t10 - 10.0 / 27.0 * t260 * t261 * t142);
    let t1111 = t1110 * t116;
    (t1102, t1110, t1111)
}
