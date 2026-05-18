//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1052/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1052<F: Float>(t3224: F, t6402: F, t2307: F, t3252: F, t2112: F, t816: F, t3258: F, t3257: F, t3287: F, t6203: F, t745: F, t851: F) -> (F, F, F, F, F, F) {
    let t9539 = F::new(7.0) / F::new(576.0) * t6402 * t3224;
    let t9540 = t3252 * t2307;
    let t9543 = t816 * t2112;
    let t9544 = t3258 * t9543;
    let t9545 = t3257 * t9544;
    let t9549 = F::new(7.0) / F::new(288.0) * t6203 * t3287;
    let t9550 = t851 * t745;
    (t9539, t9540, t9544, t9545, t9549, t9550)
}
