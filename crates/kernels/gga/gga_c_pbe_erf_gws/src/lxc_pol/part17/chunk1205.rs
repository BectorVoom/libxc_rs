//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1205/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1205<F: Float>(t353: F, t4053: F, t4386: F, t810: F, t1193: F, t2074: F, t1477: F, t326: F, t886: F, t3960: F, t1176: F, t2344: F, t923: F) -> (F, F, F, F, F, F) {
    let t51599 = t4386 * t353 * t4053 * t810;
    let t51604 = t4386 * t353 * t1193 * t2074;
    let t51649 = t326 * t1477;
    let t51650 = t51649 * t886;
    let t51651 = t51650 * t3960;
    let t51666 = t1176 * t923 * t2344;
    (t51599, t51604, t51649, t51650, t51651, t51666)
}
