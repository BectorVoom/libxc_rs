//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 508/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk508<F: Float>(t201: F, t5530: F, t1451: F, t457: F, t1162: F, t597: F, t1165: F, t461: F) -> (F, F, F, F, F) {
    let t5531 = t5530 * t201;
    let t5533 = t1451 * t457;
    let t5538 = t597 * t1162;
    let t5540 = t597 * t1165;
    let t5542 = t201 * t461;
    (t5531, t5533, t5538, t5540, t5542)
}
