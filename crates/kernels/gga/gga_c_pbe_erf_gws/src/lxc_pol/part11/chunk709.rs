//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 709/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk709<F: Float>(t11187: F, t11250: F, t11264: F, t5895: F, t5898: F, t5977: F, t5986: F, t5988: F, t5993: F, t8387: F, t8390: F, t8467: F, t1037: F, t10629: F, t3513: F, t7527: F) -> (F, F, F) {
    let t12433 = -0.59261670986728442646e-2 * t11264 + 0.26942026523072870461e-1 * t11187 + 0.94516221669423353502e-1 * t11250 + 0.19753890328909480882e-1 * t8467 - 0.11852334197345688529e-1 * t8387 - 0.14862827083471493416e-2 * t8390 - t5895 - t5898 - t5977 - t5986 + t5988 - t5993;
    let t12436 = 8.0 / 15.0 * t10629 * t1037;
    let t12438 = 8.0 / 5.0 * t7527 * t3513;
    (t12433, t12436, t12438)
}
