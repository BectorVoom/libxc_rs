//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 344/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk344<F: Float>(t385: F, t338: F, t2147: F, t2153: F, t340: F, t379: F, t382: F, t395: F, t1313: F, t2059: F, t1312: F, t2110: F, sigma0: F) -> (F, F, F, F, F) {
    let t386 = t385 < -0.66725e-1;
    let t400 = 0.0 < t338;
    let t2158 = piecewise3(t386, 0.0, 10.0 / 9.0 * t340 * t2147 * t382 - 10.0 / 27.0 * t340 * t379 * t2153);
    let t2159 = t2158 * sigma0;
    let t2160 = t2159 * t395;
    let t2163 = t1313 * t2059;
    let t2164 = t1312 * t2163;
    let t2168 = piecewise3(t400, t2110, -t2110);
    (t2159, t2160, t2163, t2164, t2168)
}
