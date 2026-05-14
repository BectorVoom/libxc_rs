//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 639/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk639<F: Float>(t27142: F, t27144: F, t1017: F, t590: F, t2185: F, t5900: F, t23657: F, t1039: F, t558: F, t23608: F, t92: F, t574: F, t23649: F, t6662: F, t2: F, t6615: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t27145 = t27142 * t27144;
    let t27147 = t1017 * t590;
    let t27149 = t2185 * t5900 * t27147;
    let t27150 = t23657 * t27149;
    let t27152 = t1039 * t558;
    let t27154 = t2185 * t5900 * t27152;
    let t27155 = t23657 * t27154;
    let t27157 = t23608 * t92;
    let t27158 = t1039 * t590;
    let t27160 = t574 * t5900 * t27158;
    let t27161 = t27157 * t27160;
    let t27163 = t23649 * t6662;
    let t27165 = t2 * t6615;
    (t27145, t27147, t27150, t27152, t27155, t27157, t27158, t27161, t27163, t27165)
}
