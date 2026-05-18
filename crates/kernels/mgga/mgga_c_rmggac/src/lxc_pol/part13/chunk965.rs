//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 965/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk965<F: Float>(t25518: F, t36: F, t5163: F, t6444: F, t8704: F, t41043: F, t793: F, t41047: F, t797: F, t2347: F, t30510: F, t36110: F, t41000: F) -> (F, F, F, F, F, F) {
    let t41176 = t25518 * t36;
    let t41177 = t41176 * t5163;
    let t41179 = t6444 * t8704;
    let t41181 = t793 * t41043;
    let t41183 = t797 * t41047;
    let t41185 = t30510 * t2347;
    let t41187 = t36110 * t41000;
    (t41177, t41179, t41181, t41183, t41185, t41187)
}
