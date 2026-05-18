//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1028/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1028<F: Float>(t12989: F, t475: F, t1: F, t12361: F, t467: F, t12362: F, t409: F, t414: F, t40: F, t460: F, t472: F, t75: F) -> (F, F, F, F, F, F) {
    let t42412 = t475 * t12989;
    let t42442 = t12361 * t1 * t467;
    let t42448 = t409 * t12362;
    let t42452 = t414 * t12362;
    let t42530 = t40 * t12361 * t460;
    let t42535 = t12361 * t75 * t472;
    (t42412, t42442, t42448, t42452, t42530, t42535)
}
