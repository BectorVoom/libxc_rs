//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 425/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk425<F: Float>(t6222: F, t6970: F, t193: F, t1196: F, t1701: F, t6027: F, t6789: F, t6793: F) -> (F, F, F, F) {
    let t6971 = t6222 * t6970;
    let t6972 = t193 * t6971;
    let t6976 = t1701 * t6027 * t1196;
    let t6979 = t6789 * t6793;
    (t6971, t6972, t6976, t6979)
}
