//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 994/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk994<F: Float>(t1882: F, t35990: F, t143100: F, t35825: F, t2: F, t35972: F, t2665: F, t6317: F, t684: F, t153372: F, t27: F, t799: F, t89: F, t35997: F, t375: F, t1486: F, t35864: F, t681: F) -> (F, F, F, F, F, F) {
    let t153388 = t1882 * t35990;
    let t153390 = t143100 * t35825;
    let t153392 = t2 * t35972;
    let t153395 = t6317 * t2665 * t153392 * t684;
    let t153399 = t89 * t27 * t799 * t153372;
    let t153402 = t89 * t375 * t35997;
    let t153405 = t1486 * t681 * t35864;
    (t153388, t153390, t153395, t153399, t153402, t153405)
}
