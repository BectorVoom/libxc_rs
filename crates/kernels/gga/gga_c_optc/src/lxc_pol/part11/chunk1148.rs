//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1148/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1148<F: Float>(t16652: F, t778: F, t17052: F, t7379: F, t888: F, t17017: F, t2586: F, t893: F, t17013: F, t7372: F, t17004: F, t17008: F) -> (F, F, F, F, F, F, F, F) {
    let t50721 = t16652 * t778;
    let t50745 = t7379 * t888 * t17052;
    let t50749 = t2586 * t17017;
    let t50750 = t893 * t50749;
    let t50758 = t7372 * t888 * t17013;
    let t50760 = t2586 * t17004;
    let t50761 = t893 * t50760;
    let t50765 = t2586 * t17008;
    (t50721, t50745, t50749, t50750, t50758, t50760, t50761, t50765)
}
