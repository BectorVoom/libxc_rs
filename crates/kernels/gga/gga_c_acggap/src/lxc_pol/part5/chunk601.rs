//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 601/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk601<F: Float>(t1388: F, t224: F, t1: F, t1378: F, t283: F, t2894: F, t1675: F, t839: F, t1674: F, t2637: F, t495: F, t694: F, t1390: F, t229: F, t276: F, t40: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t4045 = t224 * t1388;
    let t4046 = 8.0 * t4045;
    let t4047 = t1378 * t1;
    let t4048 = t4047 * t283;
    let t4049 = 0.36622894612013090108e-3 * t4048;
    let t4050 = 12.0 * t2894;
    let t4051 = t1675 * t839;
    let t4052 = t1674 * t4051;
    let t4055 = t694 * t2637 * t495;
    let t4057 = t229 * t1390;
    let t4058 = 8.0 * t4057;
    let t4059 = t1378 * t276;
    let t4060 = t40 * t4059;
    (t4046, t4047, t4048, t4049, t4050, t4052, t4055, t4057, t4058, t4059, t4060)
}
