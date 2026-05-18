//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 974/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk974<F: Float>(t2132: F, t2138: F, t3101: F, t609: F, t7941: F, t862: F, t32004: F, t7965: F, t2131: F, t2147: F, t463: F, t7997: F) -> (F, F, F, F) {
    let t32180 = F::new(0.8673628188205199462e0) * t2138 * t2132 * t609 * t3101;
    let t32181 = t862 * t7941;
    let t32183 = t32181 * t32004 * t7965;
    let t32187 = t2131 * t2147 * t7997 * t463;
    (t32180, t32181, t32183, t32187)
}
