//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 713/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk713<F: Float>(t1181: F, t1532: F, t5122: F, t1163: F, t1539: F, t372: F) -> (F, F, F) {
    let t5124 = t1181 * t1532 * t5122;
    let t5126 = F::new(0.85748036236139473944e-3) * t1163 * t5124;
    let t5127 = t1539 * t372;
    (t5124, t5126, t5127)
}
