//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 606/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk606<F: Float>(t3921: F, t848: F, t1232: F, t458: F, t2771: F, t4052: F, t1212: F, t2: F, t2681: F, t824: F, t192: F, t4129: F, t852: F) -> (F, F, F, F, F, F) {
    let t4210 = t848 * t3921;
    let t4213 = t458 * t1232;
    let t4215 = t2771 * t4052;
    let t4218 = t2 * t1212;
    let t4220 = t2681 * t4218 * t824;
    let t4224 = t192 * t852 * t4129;
    (t4210, t4213, t4215, t4218, t4220, t4224)
}
