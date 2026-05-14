//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 548/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk548<F: Float>(t378: F, t7973: F, t92: F, t7945: F, t7946: F, t7948: F, t7950: F, t7952: F, t7957: F, t7961: F, t7964: F, t7968: F, t7971: F) -> (F, F, F) {
    let t7974 = t378 * t7973;
    let t7975 = t92 * t7974;
    let t7977 = -t7945 - 4.0 / 9.0 * t7946 + 2.0 / 9.0 * t7948 - 2.0 / 3.0 * t7950 + t7952 / 3.0 - 10.0 / 27.0 * t7957 + 4.0 / 3.0 * t7961 - 2.0 / 3.0 * t7964 - 2.0 * t7968 + 2.0 * t7971 - t7975 / 3.0;
    (t7974, t7975, t7977)
}
