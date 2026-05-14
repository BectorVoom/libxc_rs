//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 540/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk540<F: Float>(t102: F, t2474: F, t159: F, t23: F, t6: F, t107: F, t821: F, t9: F) -> (F, F, F, F, F) {
    let t4858 = t102 * t2474;
    let t4863 = 1.0 / t23 / t159;
    let t4864 = t6 * t4863;
    let t4865 = t107 * t4864;
    let t4879 = 1.0 / t9 / t821;
    (t4858, t4863, t4864, t4865, t4879)
}
