//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 761/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk761<F: Float>(t336: F, t9625: F, t578: F, t137: F, t1894: F, t2263: F, t8480: F, t2068: F, t1839: F, t599: F, t1181: F, t1165: F, t604: F, t1815: F, t7413: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t9626 = t336 * t9625;
    let t9627 = t578 * t9626;
    let t9630 = t336 * t1894 * t137;
    let t9631 = t578 * t9630;
    let t9633 = t8480 * t2263;
    let t9634 = t2068 * t9633;
    let t9636 = t599 * t1839;
    let t9637 = t1181 * t9636;
    let t9638 = t2068 * t9637;
    let t9641 = t1165 * t604 * t1839;
    let t9642 = t2068 * t9641;
    let t9645 = t1165 * t604 * t1815;
    let t9646 = t7413 * t9645;
    let t9648 = t599 * t1815;
    let t9649 = t1181 * t9648;
    let t9650 = t7413 * t9649;
    (t9626, t9627, t9630, t9631, t9633, t9634, t9636, t9637, t9638, t9641, t9642, t9645, t9646, t9648, t9649, t9650)
}
