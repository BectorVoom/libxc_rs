//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 662/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk662<F: Float>(t797: F, t8700: F, t2347: F, t6444: F, t793: F, t8704: F, t851: F, t8708: F, t854: F, t8712: F, t1632: F, t649: F) -> (F, F, F, F, F, F, F, F) {
    let t8729 = t797 * t8700;
    let t8731 = t6444 * t2347;
    let t8733 = t793 * t8704;
    let t8735 = t851 * t8708;
    let t8737 = t854 * t8712;
    let t8739 = t797 * t8712;
    let t8741 = t793 * t8708;
    let t8743 = t649 * t1632;
    (t8729, t8731, t8733, t8735, t8737, t8739, t8741, t8743)
}
