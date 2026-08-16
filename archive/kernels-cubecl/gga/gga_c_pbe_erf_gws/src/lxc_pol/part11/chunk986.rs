//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 986/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk986<F: Float>(t1383: F, t169: F, t3689: F, t3373: F, t39: F, t1477: F, t1480: F, t3379: F, t551: F, t142: F, t985: F, t10207: F, t751: F) -> (F, F, F, F, F) {
    let t34254 = t169 * t3689 * t1383;
    let t34274 = t39 * t3373;
    let t34300 = t1477 * t3379 * t551 * t1480;
    let t34302 = t985 * t142;
    let t34326 = t751 * t10207;
    (t34254, t34274, t34300, t34302, t34326)
}
