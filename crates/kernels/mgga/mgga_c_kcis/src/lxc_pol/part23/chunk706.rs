//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 706/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk706<F: Float>(t589: F, t8182: F, t2069: F, t7940: F, t2253: F, t5897: F) -> (F, F, F, F) {
    let t8183 = t8182 * t589;
    let t8184 = t7940 * t2069;
    let t8185 = t5897 * t2253;
    let t8186 = t2253 * t2069;
    (t8183, t8184, t8185, t8186)
}
