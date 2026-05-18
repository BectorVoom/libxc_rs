//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 965/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk965<F: Float>(t2378: F, t3771: F, t66482: F, t1613: F, t679: F, t694: F, t21328: F, t2393: F, t21373: F, t236: F, t3724: F, t21130: F, t2382: F) -> (F, F, F, F, F) {
    let t79956 = t3771 * t66482 * t2378;
    let t79964 = t3771 * t694 * t1613 * t679;
    let t79972 = t3771 * t21328 * t2393;
    let t79997 = t3724 * t236 * t21373;
    let t80002 = t21130 * t2382;
    (t79956, t79964, t79972, t79997, t80002)
}
