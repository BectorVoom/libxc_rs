//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 549/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk549<F: Float>(t1165: F, t3174: F, t3176: F, t1163: F, t301: F, t435: F, t1160: F, t1172: F) -> (F, F, F, F) {
    let t3178 = t1165 * t3174 * t3176;
    let t3179 = t1163 * t3178;
    let t3189 = t435 * t301;
    let t3194 = t1160 * t1172;
    (t3178, t3179, t3189, t3194)
}
