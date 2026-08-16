//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 666/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk666<F: Float>(t1997: F, t9222: F, t2057: F, t5055: F, t530: F, t7894: F, t1550: F, t9005: F, t2406: F, t275: F, t1668: F, t2131: F) -> (F, F, F, F, F, F) {
    let t9223 = t9222 * t1997;
    let t9225 = t5055 * t2057;
    let t9227 = t530 * t7894;
    let t9229 = t1550 * t9005;
    let t9231 = t275 * t2406;
    let t9232 = t1668 * t2131;
    (t9223, t9225, t9227, t9229, t9231, t9232)
}
