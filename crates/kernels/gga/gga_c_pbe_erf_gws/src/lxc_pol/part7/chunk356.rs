//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 356/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk356<F: Float>(t50: F, t1412: F, t1413: F, t1416: F, t52: F, t1411: F, t59: F, zeta_threshold: F) -> (F,) {
    let t51 = t50 <= zeta_threshold;
    let t1420 = piecewise3(t51, 0.0, 4.0 / 9.0 * t1412 * t1413 + 4.0 / 3.0 * t52 * t1416);
    let t1422 = (t1411 + t1420) * t59;
    (t1422,)
}
