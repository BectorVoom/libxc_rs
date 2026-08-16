//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1115/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1115<F: Float>(t2413: F, t2739: F, t2665: F, t446: F, t10631: F, t2756: F, t2789: F, t91: F, t10622: F, t2755: F, t856: F, t10498: F, t9591: F) -> (F, F, F, F, F) {
    let t43397 = t2413 * t2739;
    let t43399 = t446 * t2665 * t43397;
    let t43403 = t91 * t10631 * t2756 * t2789;
    let t43407 = t91 * t2755 * t10622 * t856;
    let t43409 = t9591 * t10498;
    (t43397, t43399, t43403, t43407, t43409)
}
