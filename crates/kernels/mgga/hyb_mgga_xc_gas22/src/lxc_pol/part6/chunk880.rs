//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 880/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk880<F: Float>(t1047: F, t7435: F, t1031: F, t1029: F, t2727: F, t441: F, t2730: F, t453: F, t7307: F, t450: F, t2731: F, t2723: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t7436 = t7435 * t1047;
    let t7438 = F::new(1.0) * t1031 * t7436;
    let t7440 = F::new(1.0) / t2727 / t1029;
    let t7441 = t441 * t7440;
    let t7443 = F::new(1.0) / t2730 / t453;
    let t7444 = t7307 * t7443;
    let t7446 = F::cast_from(0.51726012919273400301e3_f64) * t7441 * t7444;
    let t7448 = F::new(1.0) / t2727 / t450;
    let t7449 = t441 * t7448;
    let t7450 = t7307 * t2731;
    let t7452 = F::cast_from(0.96491876992155210402e2_f64) * t7449 * t7450;
    let t7453 = t2723 * t2731;
    (t7436, t7438, t7440, t7441, t7443, t7444, t7446, t7448, t7449, t7450, t7452, t7453)
}
