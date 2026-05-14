//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 744/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk744<F: Float>(t23571: F, t5947: F, t12968: F, t27015: F, t5956: F, t13140: F, t144: F, t32995: F, t167: F, t32869: F, t574: F, t1882: F, t7409: F, t376: F, t7392: F, t89: F) -> (F, F, F, F, F, F, F, F) {
    let t33199 = t23571 * t5947;
    let t33200 = t12968 * t33199;
    let t33203 = t27015 * t5956;
    let t33204 = t13140 * t33203;
    let t33207 = t144 * t32995;
    let t33211 = t574 * t167 * t32869;
    let t33215 = t1882 * t7409 / 9.0;
    let t33218 = t89 * t376 * t7392 / 9.0;
    (t33199, t33200, t33203, t33204, t33207, t33211, t33215, t33218)
}
