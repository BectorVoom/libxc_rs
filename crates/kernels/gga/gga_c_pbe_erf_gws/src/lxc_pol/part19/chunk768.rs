//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 768/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk768<F: Float>(t1820: F, t7530: F, t1062: F, t1903: F, t7380: F, t2723: F, t582: F, t211: F, t2519: F, t713: F, t1652: F, t2615: F, t1009: F, t4991: F, t587: F, t2815: F, t586: F) -> (F, F, F, F, F, F, F, F) {
    let t7532 = 16.0 / 45.0 * t1820 * t7530;
    let t7541 = t1062 * t1903;
    let t7549 = 0.2518888888888888889e-2 * t7380;
    let t7570 = t582 * t2723;
    let t7572 = 8.0 / 45.0 * t211 * t7570;
    let t7573 = t2519 * t713;
    let t7578 = 16.0 / 135.0 * t2615 * t1652;
    let t7579 = t4991 * t1009;
    let t7580 = t587 * t7579;
    let t7582 = t2815 * t586;
    (t7532, t7541, t7549, t7572, t7573, t7578, t7580, t7582)
}
