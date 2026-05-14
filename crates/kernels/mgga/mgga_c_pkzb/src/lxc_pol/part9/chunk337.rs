//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 337/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk337<F: Float>(t24: F, t1157: F, t201: F, t977: F, t326: F, t1003: F, t821: F, zeta_threshold: F) -> (F, F, F, F) {
    let t90 = t24 <= zeta_threshold;
    let t1158 = t201 * t1157;
    let t1161 = 1.0 / t977;
    let t1162 = t326 * t1161;
    let t1165 = t821 * t1003;
    let t1167 = piecewise3(t90, 0.0, -t1165 / 3.0);
    (t1158, t1162, t1165, t1167)
}
