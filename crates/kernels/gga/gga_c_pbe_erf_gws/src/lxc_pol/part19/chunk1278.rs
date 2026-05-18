//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1278/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1278<F: Float>(t15134: F, t51563: F, t1161: F, t274: F, t1123: F, t1178: F, t13917: F, t2416: F, t938: F, t11525: F, t51066: F, t53865: F) -> (F, F, F, F) {
    let t56242 = t51563 * t15134;
    let t56246 = t274 * t1161;
    let t56250 = t13917 * t1178 * t2416 * param_a_c * t1123 * t56246 * t938;
    let t56255 = t53865 * t51066 * t11525;
    (t56242, t56246, t56250, t56255)
}
