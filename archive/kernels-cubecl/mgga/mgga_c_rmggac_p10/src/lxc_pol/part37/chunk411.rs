//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 411/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk411<F: Float>(t7756: F, t8465: F, t270: F, t575: F, t535: F, t236: F, t5605: F, t498: F, t558: F, t511: F, t1632: F, t2313: F, t458: F) -> (F, F, F, F, F, F, F) {
    let t8466 = t8465 * t7756;
    let t8475 = t575 * t270;
    let t8486 = t535 * t270;
    let t8496 = t236 * t5605;
    let t8502 = t558 * t498;
    let t8503 = t511 * t8502;
    let t8507 = t511 * t1632;
    let t8511 = t2313 * t458;
    (t8466, t8475, t8486, t8496, t8503, t8507, t8511)
}
