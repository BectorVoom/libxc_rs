//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1232/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1232<F: Float>(t2855: F, t3107: F, t26889: F, t1111: F, t11885: F, t8498: F, t8493: F, t9142: F, t140: F, t24563: F, t446: F) -> (F, F, F, F) {
    let t27152 = t3107 * t2855;
    let t27153 = t27152 * t26889;
    let t27158 = t1111 * t11885 * t8498;
    let t27167 = t1111 * t9142 * t8493;
    let t27173 = t446 * t24563 * t140;
    (t27153, t27158, t27167, t27173)
}
