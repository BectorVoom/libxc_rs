//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1332/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1332<F: Float>(t1259: F, t1306: F, t19339: F, t22688: F, t22721: F, t22724: F, t22726: F, t22729: F, t22731: F, t22733: F, t22822: F, t22902: F, t22904: F, t6359: F) -> F {
    let t23567 = -F::cast_from(6.0_f64) * t1259 * t1306 * t19339 * t6359 + t22688 - t22721 + t22724 + t22726 + t22729 + t22731 + t22733 - t22822 + t22902 - t22904;
    t23567
}
