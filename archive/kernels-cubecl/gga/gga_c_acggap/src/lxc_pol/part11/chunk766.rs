//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 766/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk766<F: Float>(t7932: F, t7965: F, t7963: F, t150: F, t187: F, t7877: F, t310: F, t609: F) -> (F, F, F, F) {
    let t7966 = t7932 * t7965;
    let t7967 = t7963 * t7966;
    let t7970 = t7877 * t150 * t187;
    let t7973 = t310 * t609;
    (t7966, t7967, t7970, t7973)
}
