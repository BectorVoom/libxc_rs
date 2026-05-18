//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 951/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk951<F: Float>(t1969: F, t39719: F, t446: F, t1971: F, t8232: F, t525: F, t7954: F, t558: F, t7955: F, t1651: F, t2075: F, t358: F, t363: F, t9007: F) -> (F, F, F, F, F, F, F) {
    let t39721 = t446 * t1969 * t39719;
    let t39723 = t8232 * t1971;
    let t39725 = t7954 * t525;
    let t39726 = t7955 * t558;
    let t39728 = t446 * t39725 * t39726;
    let t39730 = t1651 * t2075;
    let t39732 = t446 * t1969 * t39730;
    let t39735 = t9007 * t358 * t363;
    (t39721, t39723, t39726, t39728, t39730, t39732, t39735)
}
