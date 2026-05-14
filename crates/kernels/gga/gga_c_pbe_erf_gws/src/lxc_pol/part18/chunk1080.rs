//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1080/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1080<F: Float>(t2242: F, t4055: F, t2306: F, t332: F, t1477: F, t326: F, t886: F, t3960: F, t1176: F, t2344: F, t923: F) -> (F, F, F, F, F, F) {
    let t51572 = t2242 * t4055;
    let t51580 = t2306 * t332;
    let t51649 = t326 * t1477;
    let t51650 = t51649 * t886;
    let t51651 = t51650 * t3960;
    let t51666 = t1176 * t923 * t2344;
    (t51572, t51580, t51649, t51650, t51651, t51666)
}
