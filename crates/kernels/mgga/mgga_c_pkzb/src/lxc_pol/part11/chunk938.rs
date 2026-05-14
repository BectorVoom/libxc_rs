//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 938/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk938<F: Float>(t12: F, t10518: F, t11118: F, t1151: F, t1153: F, t318: F, t319: F, t3706: F, t3710: F, t201: F, t199: F, t399: F, t326: F, t427: F, t1270: F, t3719: F, t1162: F, t3949: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F, F, F, F) {
    let t84 = t12 <= zeta_threshold;
    let t203 = rho0 <= dens_threshold || t84;
    let t11125 = piecewise3(t84, 0.0, t10518);
    let t11129 = piecewise3(t203, 0.0, t11118 * t319 / 2.0 + 3.0 / 2.0 * t3706 * t1153 + 3.0 / 2.0 * t1151 * t3710 + t318 * t11125 / 2.0);
    let t11130 = t201 * t11129;
    let t11131 = t199 * t11130;
    let t11132 = 0.2390625e-1 * t11131;
    let t11133 = 1.0 / t399;
    let t11134 = t326 * t11133;
    let t11135 = t11134 * t427;
    let t11136 = 0.57375e0 * t11135;
    let t11137 = t3719 * t1270;
    let t11138 = 0.4303125e0 * t11137;
    let t11139 = t1162 * t3949;
    (t11125, t11130, t11132, t11134, t11136, t11138, t11139)
}
