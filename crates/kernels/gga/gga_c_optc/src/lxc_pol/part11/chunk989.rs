//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 989/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk989<F: Float>(t2105: F, t4668: F, t141: F, t4649: F, t4631: F, t6893: F, t4652: F, t4656: F, t2080: F, t4661: F, t4665: F, t22242: F, t4626: F, t4620: F, t6956: F, t22889: F, t4616: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t38463 = t4668 * t2105;
    let t38486 = t141 * t4649;
    let t38553 = t6893 * t4631;
    let t38668 = t6893 * t4652;
    let t38671 = t6893 * t4656;
    let t38685 = t2080 * t4661;
    let t38689 = t2080 * t4665;
    let t38749 = t22242 * t4626;
    let t38770 = t6956 * t4620;
    let t38783 = t22889 * t4616;
    (t38463, t38486, t38553, t38668, t38671, t38685, t38689, t38749, t38770, t38783)
}
