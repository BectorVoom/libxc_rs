//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 852/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk852<F: Float>(t21328: F, t2393: F, t3771: F, t21373: F, t236: F, t3724: F, t21130: F, t2382: F, t807: F, t21233: F, t213: F, t21359: F, t458: F, t21366: F, t21356: F, t21363: F) -> (F, F, F, F, F, F, F, F, F) {
    let t79972 = t3771 * t21328 * t2393;
    let t79997 = t3724 * t236 * t21373;
    let t80002 = t21130 * t2382;
    let t80003 = t807 * t80002;
    let t80012 = t213 * t21233;
    let t80029 = t458 * t21359;
    let t80031 = t458 * t21366;
    let t80087 = t458 * t21356;
    let t80089 = t458 * t21363;
    (t79972, t79997, t80002, t80003, t80012, t80029, t80031, t80087, t80089)
}
