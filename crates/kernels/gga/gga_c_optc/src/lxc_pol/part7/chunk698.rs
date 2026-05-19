//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 698/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk698<F: Float>(t1864: F, t587: F, t6407: F, t601: F, t1874: F, t539: F, t544: F, t1963: F, t1975: F, t712: F, t1906: F, t75: F) -> (F, F, F, F, F, F, F, F) {
    let t6617 = t1864 * t6407 * t587;
    let t6619 = F::cast_from(0.35089340384731224426e1_f64) * t601 * t6617;
    let t6620 = t539 * t1874;
    let t6621 = F::new(24.0) * t6620;
    let t6622 = t544 * t1874;
    let t6623 = F::new(24.0) * t6622;
    let t6624 = t539 * t1963;
    let t6625 = F::new(12.0) * t6624;
    let t6626 = t544 * t1963;
    let t6627 = F::new(12.0) * t6626;
    let t6628 = t712 * t1975;
    let t6632 = t1906 * t75;
    (t6617, t6619, t6621, t6623, t6625, t6627, t6628, t6632)
}
