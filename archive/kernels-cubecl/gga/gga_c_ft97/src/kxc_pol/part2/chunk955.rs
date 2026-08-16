//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 955/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk955<F: Float>(t13433: F, t274: F, t683: F, t3750: F, t688: F, t231: F, t1095: F, t703: F, t10328: F, t2417: F, t230: F, t2380: F, t801: F) -> (F, F, F, F, F, F) {
    let t14818 = t683 * t13433 * t274;
    let t14825 = t3750 * t688;
    let t14826 = t14825 * t274;
    let t14827 = t231 * t14826;
    let t14832 = t703 * t1095;
    let t14833 = t14832 * t688;
    let t14834 = t14833 * t10328;
    let t14839 = t231 * t1095 * t2417 * t274;
    let t14842 = t230 * t1095;
    let t14844 = t2380 * t801 * t274;
    (t14818, t14827, t14834, t14839, t14842, t14844)
}
