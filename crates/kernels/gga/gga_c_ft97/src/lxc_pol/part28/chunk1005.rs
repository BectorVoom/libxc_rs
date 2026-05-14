//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1005/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1005<F: Float>(t28: F, t34918: F, t586: F, t5890: F, t590: F, t446: F, t5842: F, t6630: F, t9432: F, t147730: F, t5899: F, t95344: F, t34943: F, t375: F, t89: F, t147590: F, t27: F, t526: F) -> (F, F, F, F, F) {
    let t148621 = t5890 * t28 * t586 * t34918 * t590;
    let t148625 = t446 * t9432 * t6630 * t5842;
    let t148629 = t5899 * t95344 * t147730;
    let t148632 = t89 * t375 * t34943;
    let t148636 = t89 * t27 * t526 * t147590;
    (t148621, t148625, t148629, t148632, t148636)
}
