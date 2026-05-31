//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1135/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1135<F: Float>(t4475: F, t483: F, t1112: F, t11217: F, t495: F, t1100: F, t462: F, t7426: F, t7438: F, t7446: F, t7452: F, t7456: F, t7459: F, t7463: F, t7466: F, t7503: F, t7506: F, t7518: F, t7522: F, t7523: F) -> (F, F, F, F) {
    let t11232 = t4475 * t483;
    let t11233 = t11232 * t1112;
    let t11235 = t11217 * t495;
    let t11237 = t4475 * t1100;
    let t11238 = t462 * t11237;
    let t11241 = -F::cast_from(0.5848223622634646207e0_f64) * t11233 + t7426 + t462 * t11235 + t11238 - t7503 + F::cast_from(20.0_f64) * t7506 + t7438 + t7446 - t7452 + t7456 - t7459 - t7463 - t7466 - t7518 - t7522 + F::cast_from(0.24415263074675393405e-3_f64) * t7523;
    (t11232, t11235, t11237, t11241)
}
