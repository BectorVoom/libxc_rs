//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1148/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1148<F: Float>(t100234: F, t446: F, t7793: F, t28: F, t3157: F, t469: F, t5617: F, t5665: F, t11392: F, t1307: F, t1317: F, t1800: F, t11437: F, t93416: F, t37305: F, t1882: F, t25969: F) -> (F, F, F, F, F, F, F) {
    let t100236 = t446 * t7793 * t100234;
    let t100241 = t5665 * t28 * t469 * t5617 * t3157;
    let t100243 = t1307 * t11392;
    let t100246 = t1317 * t28 * t1800 * t100243;
    let t100248 = t93416 * t11437;
    let t100250 = t446 * t37305 * t100248;
    let t100252 = t1882 * t25969;
    (t100236, t100241, t100243, t100246, t100248, t100250, t100252)
}
