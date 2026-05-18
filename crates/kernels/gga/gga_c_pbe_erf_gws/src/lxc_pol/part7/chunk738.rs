//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 738/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk738<F: Float>(t312: F, t6067: F, t4341: F, t4345: F, t4349: F, t4499: F, t4503: F, t4506: F, t4513: F, t4539: F, t4542: F, t4546: F, t4548: F, t4603: F, t4744: F) -> (F, F) {
    let t6068 = t6067 * t312;
    let t6069 = t4341 - t4345 - t4349 + t4499 + t4503 - t4506 - t4513 + t4539 + t4542 - t4546 - t4548 - t6068 + t4603 + t4744;
    (t6068, t6069)
}
