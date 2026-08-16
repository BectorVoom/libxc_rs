//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2510/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2510<F: Float>(t136: F, t43761: F, t50924: F, t14778: F, t699: F, t11219: F, t50910: F, t50915: F, t11153: F, t1229: F, t45971: F, t47774: F) -> (F, F, F, F, F, F) {
    let t50976 = t136 * t43761 * t50924;
    let t50978 = t699 * t14778;
    let t50987 = t136 * t11219 * t50910;
    let t50990 = t136 * t11219 * t50915;
    let t50992 = t1229 * t11153;
    let t50994 = t47774 * t50992 * t45971;
    (t50976, t50978, t50987, t50990, t50992, t50994)
}
