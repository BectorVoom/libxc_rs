//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1242/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1242<F: Float>(t2770: F, t6318: F, t113458: F, t28736: F, t684: F, t25026: F, t28738: F, t458: F, t2413: F, t7036: F, t24976: F, t6317: F, t112742: F, t99391: F, t2409: F, t24980: F, t24981: F, t7062: F) -> (F, F, F, F, F, F, F, F) {
    let t113459 = t2770 * t6318;
    let t113462 = t113458 * t113459 * t28736 * t684;
    let t113465 = t25026 * t458 * t28738;
    let t113466 = t113465 / 4.0;
    let t113467 = t7036 * t2413;
    let t113469 = t6317 * t24976 * t113467;
    let t113472 = t6317 * t99391 * t112742;
    let t113476 = t24980 * t24981 * t7062 * t2409;
    (t113459, t113462, t113465, t113466, t113467, t113469, t113472, t113476)
}
