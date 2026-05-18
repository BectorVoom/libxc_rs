//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 602/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk602<F: Float>(t1970: F, t8497: F, t2305: F, t7244: F, t498: F, t558: F, t511: F, t7231: F, t3351: F, t1632: F, t3352: F, t2313: F, t458: F) -> (F, F, F, F, F, F, F) {
    let t8498 = t1970 * t8497;
    let t8500 = t7244 * t2305;
    let t8502 = t558 * t498;
    let t8503 = t511 * t8502;
    let t8504 = t7231 * t8503;
    let t8505 = t3351 * t8504;
    let t8507 = t511 * t1632;
    let t8508 = t3352 * t8507;
    let t8509 = t3351 * t8508;
    let t8511 = t2313 * t458;
    (t8498, t8500, t8504, t8505, t8508, t8509, t8511)
}
