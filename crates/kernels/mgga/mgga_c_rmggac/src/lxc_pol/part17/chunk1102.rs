//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1102/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1102<F: Float>(t38530: F, t8427: F, t46522: F, t8630: F, t36639: F, t9713: F, t2057: F, t31273: F, t2868: F, t8413: F, t46058: F, t739: F) -> (F, F, F, F, F, F) {
    let t48000 = t38530 * t8427;
    let t48009 = t8630 * t46522;
    let t48011 = t36639 * t9713;
    let t48014 = t31273 * t2057;
    let t48017 = t2868 * t8413;
    let t48022 = t739 * t46058;
    (t48000, t48009, t48011, t48014, t48017, t48022)
}
