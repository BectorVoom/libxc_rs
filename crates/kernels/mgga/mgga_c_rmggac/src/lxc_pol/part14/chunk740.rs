//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 740/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk740<F: Float>(t16503: F, t3369: F, t352: F, t38422: F, t38444: F, t34761: F, t8432: F, t205: F, t24985: F, t3350: F, t671: F, t1971: F, t236: F, t5561: F, t16155: F, t8516: F, t8519: F) -> (F, F, F, F) {
    let t38448 = t16503 * t3369 * t38422 * t38444 * t352;
    let t38450 = t34761 * t8432;
    let t38454 = t671 * t24985 * t205 * t3350;
    let t38457 = t38454 * t1971 * t236 * t5561;
    let t38460 = t8516 * t16155 * t8519;
    (t38448, t38450, t38457, t38460)
}
