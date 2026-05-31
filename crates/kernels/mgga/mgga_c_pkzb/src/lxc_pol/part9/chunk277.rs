//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 277/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk277<F: Float>(t819: F, t845: F, t826: F, t837: F, t842: F, t849: F) -> (F, F, F) {
    let t884 = F::cast_from(0.301925e0_f64) * t819;
    let t887 = F::cast_from(0.82785e-1_f64) * t845;
    let t889 = F::cast_from(0.258925e1_f64) * t837 - t884 + F::cast_from(0.905775e0_f64) * t826 + F::cast_from(0.16504875e0_f64) * t842 - t887 + F::cast_from(0.248355e0_f64) * t849;
    (t884, t887, t889)
}
