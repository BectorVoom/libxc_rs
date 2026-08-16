//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 489/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk489<F: Float>(t221: F, t446: F, t6172: F, t1888: F, t476: F, t209: F, t1867: F) -> (F, F, F, F) {
    let t6174 = t221 * t6172 * t446;
    let t6177 = t1888 * t476;
    let t6178 = t6177 * t209;
    let t6179 = t221 * t6178;
    let t6182 = t1867 * t209;
    (t6174, t6178, t6179, t6182)
}
