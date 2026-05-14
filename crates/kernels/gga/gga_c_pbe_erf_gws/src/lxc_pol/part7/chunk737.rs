//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 737/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk737<F: Float>(t343: F, t6439: F, t858: F, t867: F, t866: F, t2164: F, t2197: F, t2192: F, t2074: F, t810: F) -> (F, F, F, F, F, F) {
    let t6440 = t6439 * t343;
    let t6442 = t867 * t858 * t6440;
    let t6444 = t866 * t6442 / 96.0;
    let t6445 = t2164 * t2197;
    let t6446 = 7.0 / 96.0 * t6445;
    let t6447 = t2164 * t2192;
    let t6448 = 7.0 / 96.0 * t6447;
    let t6449 = t2074 * t810;
    (t6440, t6442, t6444, t6446, t6448, t6449)
}
