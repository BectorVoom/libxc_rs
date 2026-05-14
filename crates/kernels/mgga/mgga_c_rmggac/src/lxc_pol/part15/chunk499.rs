//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 499/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk499<F: Float>(t6567: F, t6616: F, t1810: F, t941: F, t1664: F, t574: F, t271: F, t830: F) -> (F, F, F, F) {
    let t6617 = t6567 + t6616;
    let t6624 = t941 * t1810;
    let t6627 = t1664 * t574;
    let t7184 = t830 * t271;
    (t6617, t6624, t6627, t7184)
}
