//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 978/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk978<F: Float>(t23244: F, t925: F, t1902: F, t3052: F, t5630: F, t1882: F, t6492: F, t452: F, t5750: F, t942: F, t23265: F, t3204: F, t8557: F, t3113: F, t11854: F, t379: F, t447: F, t6564: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t26184 = t23244 * t925;
    let t26185 = t1902 * t26184;
    let t26188 = t5630 * t3052;
    let t26189 = t1902 * t26188;
    let t26192 = t1882 * t6492;
    let t26195 = t452 * t5750 * t942;
    let t26198 = t23265 * t3204;
    let t26199 = t8557 * t26198;
    let t26202 = t23265 * t3113;
    let t26203 = t11854 * t26202;
    let t26207 = t447 * t6564 * t379;
    (t26184, t26185, t26188, t26189, t26192, t26195, t26198, t26199, t26202, t26203, t26207)
}
