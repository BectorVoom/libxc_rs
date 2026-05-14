//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 691/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk691<F: Float>(t30635: F, t901: F, t2389: F, t9298: F, t12448: F, t2464: F, t2487: F, t29853: F, t883: F, t1538: F, t9267: F, t29984: F, t4782: F, t9272: F, t1415: F, t7030: F, t9297: F) -> (F, F, F, F, F, F, F) {
    let t40023 = t30635 * t901;
    let t40073 = t9298 * t2389;
    let t40076 = t2487 * t2464 * t12448;
    let t40088 = t883 * t29853;
    let t40090 = t9267 * t1538 * t40088;
    let t40103 = t9272 * t4782 * t883 * t29984;
    let t40106 = t1415 * t9297 * t7030;
    (t40023, t40073, t40076, t40088, t40090, t40103, t40106)
}
