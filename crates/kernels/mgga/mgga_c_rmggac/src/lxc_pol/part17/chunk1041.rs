//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1041/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1041<F: Float>(t40278: F, t8443: F, t1704: F, t352: F, t2186: F, t9795: F, t6491: F, t668: F, t7244: F, t9985: F, t3351: F, t3352: F, t511: F, t6449: F) -> (F, F, F, F, F, F) {
    let t47119 = t40278 * t8443;
    let t47124 = t1704 * t352;
    let t47133 = t2186 * t9795;
    let t47135 = t6491 * t668;
    let t47138 = t7244 * t9985;
    let t47142 = t3351 * t3352 * t511 * t6449;
    (t47119, t47124, t47133, t47135, t47138, t47142)
}
