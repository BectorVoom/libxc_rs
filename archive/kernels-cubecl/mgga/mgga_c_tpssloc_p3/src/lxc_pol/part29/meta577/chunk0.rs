//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1994/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1994<F: Float>(t22674: F, t22686: F, t80681: F, t22663: F, t6883: F, t225: F, t22624: F, t22622: F, t214: F, t3879: F, t22675: F, t22724: F) -> (F, F, F, F, F, F) {
    let t80683 = t80681 * t22674 * t22686;
    let t80689 = t6883 * t22663;
    let t80699 = t22624 * t225;
    let t80704 = t22622 * t225;
    let t80707 = t214 * t3879;
    let t80711 = t22724 * t22675;
    (t80683, t80689, t80699, t80704, t80707, t80711)
}
