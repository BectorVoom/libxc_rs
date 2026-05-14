//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 374/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk374<F: Float>(t2181: F, t2193: F, t2197: F, t2185: F, t2187: F) -> (F, F) {
    let t2201 = -0.34752604166666666667e-3 * t2193 * t2197 + 0.17411041666666666666e-2 * t2181;
    let t2205 = 0.9375e-1 * t2185 - 0.20234375e-1 * t2187;
    (t2201, t2205)
}
