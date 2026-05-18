//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 791/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk791<F: Float>(t2003: F, t53: F, t179: F, t1885: F, t299: F, t2002: F, t208: F) -> (F, F, F) {
    let t5627 = t53 * t2003;
    let t5629 = t179 * t5627 * t1885;
    let t5630 = t299 * t5629;
    let t5633 = F::new(1.0) / t2002 / t208;
    (t5629, t5630, t5633)
}
