//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1018/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1018<F: Float>(t24642: F, t27811: F, t27826: F, t27873: F, t27876: F, t31011: F, t31017: F, t31022: F, t31027: F, t31032: F, t31034: F, t31039: F, t31043: F, t31046: F, t31050: F, t31054: F) -> (F,) {
    let t31096 = 2.0 / 9.0 * t31011 - t24642 + 2.0 / 9.0 * t27811 + 2.0 / 3.0 * t31017 + t31022 / 12.0 + t31027 / 6.0 + t31032 / 3.0 - 4.0 / 9.0 * t31034 - t31039 - t31043 / 3.0 - 2.0 * t31046 + 4.0 / 3.0 * t31050 + 2.0 / 3.0 * t31054 - 4.0 / 9.0 * t27826 - t27873 / 18.0 - 2.0 / 9.0 * t27876;
    (t31096,)
}
