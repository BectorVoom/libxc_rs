//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3433/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3433<F: Float>(t2875: F, t2924: F, t6142: F, t15380: F, t52645: F, t19330: F, t2918: F, t11385: F, t11387: F, t6141: F, t15098: F, t15421: F) -> (F, F, F, F, F) {
    let t64465 = F::new(6.0) * t2924 * t6142 * t2875;
    let t64467 = F::new(24.0) * t52645 * t15380;
    let t64471 = F::cast_from(0.16081979498692535067e2_f64) * t2924 * t19330 * t2918;
    let t64475 = F::cast_from(0.51726012919273400301e3_f64) * t11385 * t6141 * t11387 * t2875;
    let t64483 = F::new(12.0) * t15421 * t15098;
    (t64465, t64467, t64471, t64475, t64483)
}
