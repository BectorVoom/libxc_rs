//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 541/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk541<F: Float>(t4030: F, t2635: F, t2644: F, t2835: F, t1390: F, t224: F, t2841: F, t2843: F, t2845: F, t2847: F, t1388: F, t1: F, t1378: F, t283: F, t2894: F, t229: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t4031 = 0.24415263074675393405e-3 * t4030;
    let t4032 = 24.0 * t2635;
    let t4036 = 2.0 * t2644;
    let t4038 = 0.23392894490538584828e1 * t2835;
    let t4039 = t224 * t1390;
    let t4040 = 8.0 * t4039;
    let t4041 = 16.0 * t2841;
    let t4042 = 4.0 * t2843;
    let t4043 = 4.0 * t2845;
    let t4044 = 32.0 * t2847;
    let t4045 = t224 * t1388;
    let t4046 = 8.0 * t4045;
    let t4047 = t1378 * t1;
    let t4048 = t4047 * t283;
    let t4049 = 0.36622894612013090108e-3 * t4048;
    let t4050 = 12.0 * t2894;
    let t4057 = t229 * t1390;
    (t4031, t4032, t4036, t4038, t4040, t4041, t4042, t4043, t4044, t4046, t4049, t4050, t4057)
}
