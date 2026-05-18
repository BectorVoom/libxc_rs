//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 956/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk956<F: Float>(t1614: F, t3858: F, t12224: F, t557: F, t1605: F, t848: F, t1308: F, t3883: F, t4119: F, t857: F, t12200: F, t4131: F, t880: F) -> (F, F, F, F, F, F, F) {
    let t15230 = t3858 * t1614;
    let t15232 = t12224 * t557;
    let t15234 = t848 * t1605;
    let t15238 = t1308 * t3883;
    let t15247 = t857 * t4119;
    let t15249 = t12200 * t557;
    let t15251 = t4131 * t880;
    (t15230, t15232, t15234, t15238, t15247, t15249, t15251)
}
