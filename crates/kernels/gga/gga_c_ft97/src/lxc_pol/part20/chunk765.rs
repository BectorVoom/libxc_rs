//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 765/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk765<F: Float>(t24438: F, t6110: F, t684: F, t24437: F, t2354: F, t2409: F, t6119: F, t6118: F, t91: F, t9890: F, t26: F, t1424: F, t2476: F, t743: F, t193: F, t6109: F, t6111: F, t681: F) -> (F, F, F, F, F, F, F, F, F) {
    let t24440 = t24438 * t6110 * t684;
    let t24441 = t24437 * t24440;
    let t24444 = t2354 * t6119 * t2409;
    let t24445 = t6118 * t24444;
    let t24447 = t91 * t9890;
    let t24448 = t24447 * t26;
    let t24449 = t1424 * t2476;
    let t24450 = t743 * t24449;
    let t24452 = t24448 * t193 * t24450;
    let t24455 = t6109 * t681 * t6111;
    (t24440, t24441, t24444, t24445, t24447, t24448, t24450, t24452, t24455)
}
