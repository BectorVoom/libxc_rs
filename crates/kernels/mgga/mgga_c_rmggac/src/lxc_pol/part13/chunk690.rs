//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 690/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk690<F: Float>(t1997: F, t9222: F, t2057: F, t5055: F, t1550: F, t9005: F, t1990: F, t8571: F, t2212: F, t5928: F, t2228: F, t558: F) -> (F, F, F, F, F, F) {
    let t9223 = t9222 * t1997;
    let t9225 = t5055 * t2057;
    let t9229 = t1550 * t9005;
    let t9236 = t8571 * t1990;
    let t9300 = t5928 * t2212;
    let t9302 = t2228 * t558;
    (t9223, t9225, t9229, t9236, t9300, t9302)
}
