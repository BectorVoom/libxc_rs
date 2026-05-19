//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 577/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk577<F: Float>(t3197: F, t286: F, t3092: F, t708: F, t1687: F, t3098: F, t129: F, t1692: F, t1685: F, t3097: F, t3091: F, t713: F) -> (F, F, F, F, F, F) {
    let pi = F::cast_from(M_PI);
    let t3198 = F::cast_from(0.38342925953920749676e0_f64) * t3197;
    let t3216 = t3092 * t286 * t708;
    let t3218 = t3098 * t1687;
    let t3220 = t1692 * t129;
    let t3221 = t3097 * t1685;
    let t3222 = t3221 * pi;
    let t3223 = t3220 * t3222;
    let t3225 = t713 * t3091;
    (t3198, t3216, t3218, t3220, t3223, t3225)
}
