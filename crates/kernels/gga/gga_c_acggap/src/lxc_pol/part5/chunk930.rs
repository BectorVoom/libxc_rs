//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 930/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk930<F: Float>(t1165: F, t1532: F, t1539: F, t3194: F, t839: F, t1163: F, t1181: F, t4289: F, t5122: F, t1077: F, t4298: F, t5127: F, t1008: F, t4552: F, t5260: F, t3409: F, t4983: F) -> (F, F, F, F, F, F, F) {
    let t17441 = t3194 * t1165 * t1532 * t1539 * t839;
    let t17445 = t1163 * t1181 * t4289 * t5122;
    let t17450 = t1163 * t1181 * t1532 * t1539 * t1077;
    let t17454 = t1163 * t1165 * t4298 * t5127;
    let t17468 = t1008 * t4552;
    let t17480 = t1008 * t5260;
    let t17484 = t3409 * t4983;
    (t17441, t17445, t17450, t17454, t17468, t17480, t17484)
}
