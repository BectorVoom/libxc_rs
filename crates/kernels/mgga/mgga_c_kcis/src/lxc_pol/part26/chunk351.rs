//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 351/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk351<F: Float>(t2066: F, t578: F, t2036: F, t2040: F, t2044: F, t2048: F, t2052: F, t2056: F, t2063: F) -> (F, F) {
    let t2067 = t578 * t2066;
    let t2069 = t2036 / 16.0 - t2040 / 16.0 - t2044 / 6.0 + t2048 / 24.0 - t2052 / 256.0 + t2056 / 256.0 + t2063 / 48.0 - t2067 / 192.0;
    (t2067, t2069)
}
