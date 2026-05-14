//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 364/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk364<F: Float>(t2147: F, t2154: F, t2161: F, t2165: F, t228: F, t899: F, t209: F, t9: F, t445: F, t447: F) -> (F, F, F) {
    let t2167 = t2161 * t228 - t2165 * t899 - t2147 + t2154;
    let t2194 = t209 * t9;
    let t2233 = t445 * t447;
    (t2167, t2194, t2233)
}
