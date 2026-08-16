//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 619/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk619<F: Float>(t173: F, t2440: F, t420: F, t9651: F, t1103: F, t228: F, t231: F, t625: F, t1123: F, t626: F, t701: F, t1152: F, t1771: F) -> (F, F, F, F, F, F) {
    let t13598 = t173 * t2440;
    let t13605 = t420 * t9651;
    let t13643 = t228 * t1103 * t625 * t231;
    let t13647 = t626 * t1123;
    let t13648 = t701 * t13647;
    let t13680 = t1771 * t1152;
    (t13598, t13605, t13643, t13647, t13648, t13680)
}
