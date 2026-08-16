//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 815/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk815<F: Float>(t209: F, t333: F, t16503: F, t3369: F, t352: F, t38422: F, t34761: F, t8432: F, t205: F, t24985: F, t3350: F, t671: F) -> (F, F, F, F) {
    let t38444 = t209 * t333;
    let t38448 = t16503 * t3369 * t38422 * t38444 * t352;
    let t38450 = t34761 * t8432;
    let t38454 = t671 * t24985 * t205 * t3350;
    (t38444, t38448, t38450, t38454)
}
