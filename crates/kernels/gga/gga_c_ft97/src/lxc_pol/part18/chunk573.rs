//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 573/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk573<F: Float>(t1589: F, t375: F, t89: F, t1636: F, t355: F, t364: F, t1546: F, t1581: F, t1554: F, t1560: F, t1642: F, t369: F) -> (F, F, F, F, F, F, F) {
    let t7771 = t89 * t375 * t1589;
    let t7773 = t1636 * t355;
    let t7775 = t89 * t7773 * t364;
    let t7778 = t89 * t1546 * t1581;
    let t7780 = t375 * t1554;
    let t7782 = t89 * t7780 * t1560;
    let t7793 = t1642 * t369;
    (t7771, t7773, t7775, t7778, t7780, t7782, t7793)
}
