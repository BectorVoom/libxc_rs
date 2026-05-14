//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 596/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk596<F: Float>(t1441: F, t5886: F, t1411: F, t2218: F, t3521: F, t3530: F, t459: F, t5671: F, t1175: F, t2075: F, t3539: F, t1364: F, t3544: F, t1422: F, t5676: F, t119: F, t179: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t5887 = t5886 * t1441;
    let t5888 = t1411 * t5887;
    let t5893 = t3521 * t2218;
    let t5895 = t3530 * t459;
    let t5896 = t5895 * t5671;
    let t5900 = t3539 * t2075 * t1175;
    let t5904 = t3544 * t2075 * t1364;
    let t5907 = t1422 * t459;
    let t5908 = t5907 * t5676;
    let t5911 = t179 * t119;
    (t5887, t5888, t5893, t5895, t5896, t5900, t5904, t5907, t5908, t5911)
}
