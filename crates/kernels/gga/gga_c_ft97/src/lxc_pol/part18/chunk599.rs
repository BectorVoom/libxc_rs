//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 599/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk599<F: Float>(t1637: F, t482: F, t89: F, t100: F, t8326: F, t1822: F, t1882: F, t1863: F, t104: F, t7943: F, t1786: F, t488: F) -> (F, F, F, F, F, F) {
    let t8516 = t89 * t1637 * t482;
    let t8518 = t8326 * t100;
    let t8523 = t1882 * t1822;
    let t8526 = t1882 * t1863;
    let t8534 = 28.0 / 81.0 * t89 * t7943 * t104;
    let t8557 = t1786 * t488;
    (t8516, t8518, t8523, t8526, t8534, t8557)
}
