//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 849/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk849<F: Float>(t32338: F, t378: F, t1286: F, t32379: F, t376: F, t32355: F, t1637: F, t7213: F, t22892: F, t7162: F, t32365: F, t487: F, t1851: F, t7264: F, t137089: F, t137197: F) -> (F, F, F, F, F, F, F, F, F) {
    let t137488 = t378 * t32338;
    let t137497 = t1286 * t376 * t32379;
    let t137525 = t378 * t32355;
    let t137531 = 2.0 / 27.0 * t1286 * t1637 * t7213;
    let t137547 = t7162 * t22892;
    let t137561 = t32365 * t487;
    let t137564 = t7264 * t1851;
    let t137623 = 10.0 / 9.0 * t137089;
    let t137652 = 4.0 / 9.0 * t137197;
    (t137488, t137497, t137525, t137531, t137547, t137561, t137564, t137623, t137652)
}
