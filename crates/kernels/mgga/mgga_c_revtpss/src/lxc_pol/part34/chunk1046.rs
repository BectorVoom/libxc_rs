//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1046/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1046<F: Float>(t1651: F, t7810: F, t7145: F, t1976: F, t6392: F, t7160: F, t1668: F, t7817: F, t1089: F, t7821: F, t1646: F, t6350: F, t25464: F, t7828: F, t1972: F, t6317: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t29727 = t7810 * t1651;
    let t29728 = t7145 * t29727;
    let t29731 = t1976 * t6392;
    let t29732 = t7160 * t29731;
    let t29739 = t7817 * t1668;
    let t29740 = t29739 * t1089;
    let t29743 = t7821 * t1668;
    let t29744 = t29743 * t1089;
    let t29747 = t7810 * t1646;
    let t29748 = t7145 * t29747;
    let t29751 = t1976 * t6350;
    let t29752 = t25464 * t29751;
    let t29759 = t7828 * t1668;
    let t29760 = t29759 * t1089;
    let t29779 = t6317 * t1972;
    (t29727, t29728, t29731, t29732, t29740, t29744, t29747, t29748, t29751, t29752, t29759, t29760, t29779)
}
