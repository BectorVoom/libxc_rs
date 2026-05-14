//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 495/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk495<F: Float>(t2628: F, t958: F, t1457: F, t2582: F, t2571: F, t723: F, t1445: F, t2541: F, t313: F, t1645: F, t740: F) -> (F, F, F, F, F, F) {
    let t2629 = t958 * t2628;
    let t2631 = t1457 * t2582;
    let t2634 = t2571 * t723;
    let t2635 = t1445 * t2634;
    let t2638 = t313 * t2541;
    let t2639 = t1645 * t740;
    (t2629, t2631, t2634, t2635, t2638, t2639)
}
