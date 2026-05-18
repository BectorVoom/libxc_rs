//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 806/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk806<F: Float>(t2634: F, t4614: F, t1890: F, t7291: F, t590: F, t5241: F, t739: F, t7068: F, t2582: F, t4673: F, t1457: F, t7132: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t7656 = t4614 * t2634;
    let t7659 = t1890 * t7291;
    let t7660 = t7659 * t590;
    let t7663 = t5241 * t7291;
    let t7664 = t7663 * t590;
    let t7667 = t739 * t7291;
    let t7668 = t7667 * t590;
    let t7671 = t739 * t7068;
    let t7672 = t7671 * t590;
    let t7675 = t1890 * t7068;
    let t7676 = t7675 * t590;
    let t7679 = t4673 * t2582;
    let t7682 = t1457 * t7132;
    (t7656, t7659, t7660, t7664, t7667, t7668, t7671, t7672, t7676, t7679, t7682)
}
