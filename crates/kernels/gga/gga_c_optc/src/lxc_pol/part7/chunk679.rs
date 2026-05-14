//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 679/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk679<F: Float>(t1864: F, t587: F, t6407: F, t601: F, t1874: F, t539: F, t544: F, t1963: F, t1975: F, t712: F, t1906: F, t75: F, t603: F, t6424: F, t6427: F, t1986: F, t1998: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t6617 = t1864 * t6407 * t587;
    let t6619 = 0.35089340384731224426e1 * t601 * t6617;
    let t6620 = t539 * t1874;
    let t6621 = 24.0 * t6620;
    let t6622 = t544 * t1874;
    let t6623 = 24.0 * t6622;
    let t6624 = t539 * t1963;
    let t6625 = 12.0 * t6624;
    let t6626 = t544 * t1963;
    let t6627 = 12.0 * t6626;
    let t6628 = t712 * t1975;
    let t6632 = t1906 * t75;
    let t6633 = t6632 * t603;
    let t6634 = 0.17544670192365612213e1 * t6633;
    let t6636 = t6424 * t6407 * t6427;
    let t6638 = 0.1025389702100779493e4 * t601 * t6636;
    let t6639 = t1986 * t1998;
    (t6617, t6619, t6621, t6623, t6625, t6627, t6628, t6632, t6634, t6636, t6638, t6639)
}
