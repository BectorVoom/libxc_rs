//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1046/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1046<F: Float>(t2318: F, t6497: F, t220: F, t2326: F, t4400: F, t6187: F, t1591: F, t7706: F, t14909: F, t3952: F, t1581: F, t25312: F, t1312: F, t14995: F, t25342: F, t14935: F) -> (F, F, F, F, F, F, F, F) {
    let t27810 = t2318 * t6497;
    let t27812 = t220 * t2326;
    let t27813 = t4400 * t27812;
    let t27814 = t6187 * t27813;
    let t27817 = t7706 * t1591;
    let t27818 = t14909 * t27817;
    let t27819 = t3952 * t27818;
    let t27833 = t1581 * t25312;
    let t27834 = t1312 * t27833;
    let t27839 = t14995 * t25342;
    let t27840 = t3952 * t27839;
    let t27843 = t14935 * t25342;
    (t27810, t27812, t27814, t27817, t27819, t27834, t27840, t27843)
}
