//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 1009/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk1009<F: Float>(t41043: F, t793: F, t41047: F, t797: F, t2347: F, t30510: F, t36110: F, t41000: F, t36103: F, t41150: F, t41027: F, t2350: F, t26531: F) -> (F, F, F, F, F, F, F) {
    let t41181 = t793 * t41043;
    let t41183 = t797 * t41047;
    let t41185 = t30510 * t2347;
    let t41187 = t36110 * t41000;
    let t41189 = t36103 * t41150;
    let t41191 = t793 * t41027;
    let t41193 = t26531 * t2350;
    (t41181, t41183, t41185, t41187, t41189, t41191, t41193)
}
