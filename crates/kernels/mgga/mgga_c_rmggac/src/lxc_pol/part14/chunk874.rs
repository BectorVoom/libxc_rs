//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 874/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk874<F: Float>(t39064: F, t7788: F, t2347: F, t866: F, t262: F, t2350: F, t876: F, t36274: F, t38569: F, t7782: F, t794: F, t35810: F, t321: F, t8712: F, t7785: F, t839: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t40881 = t7788 * t39064;
    let t40883 = t2347 * t866;
    let t40884 = t262 * t40883;
    let t40885 = t7788 * t40884;
    let t40887 = t2350 * t876;
    let t40888 = t262 * t40887;
    let t40889 = t36274 * t40888;
    let t40891 = t7782 * t38569;
    let t40893 = t2350 * t794;
    let t40894 = t262 * t40893;
    let t40895 = t35810 * t40894;
    let t40897 = t8712 * t321;
    let t40898 = t262 * t40897;
    let t40899 = t7785 * t40898;
    let t40901 = t2350 * t839;
    (t40881, t40883, t40884, t40885, t40887, t40888, t40889, t40891, t40893, t40894, t40895, t40897, t40898, t40899, t40901)
}
