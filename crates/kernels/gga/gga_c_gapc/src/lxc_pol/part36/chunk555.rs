//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 555/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk555<F: Float>(t3254: F, t3255: F, t1061: F, t2452: F, t2456: F, t3239: F, t1055: F, t644: F, t311: F, t442: F, t906: F) -> (F, F, F, F, F, F, F) {
    let t3256 = t3254 * t3255;
    let t3258 = t1061 * t2452;
    let t3259 = t3239 * t2456;
    let t3260 = t3258 * t3259;
    let t3271 = t1055 * t644;
    let t3272 = t311 * t3271;
    let t3273 = t442 * t906;
    (t3256, t3258, t3259, t3260, t3271, t3272, t3273)
}
