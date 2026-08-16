//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1138/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1138<F: Float>(t125: F, t8536: F, t8538: F, t86: F, t2421: F, t7603: F, t137: F, t8963: F, t26446: F, t710: F, t8999: F, t754: F, t8750: F) -> (F, F, F, F, F, F) {
    let t91913 = t86 * t125 * t8536 * t8538;
    let t91916 = t86 * t2421 * t7603;
    let t91919 = t86 * t8963 * t137;
    let t91922 = t86 * t710 * t26446;
    let t91925 = t86 * t8999 * t137;
    let t91929 = t86 * t125 * t754 * t8750;
    (t91913, t91916, t91919, t91922, t91925, t91929)
}
