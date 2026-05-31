//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1643/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1643<F: Float>(t11294: F, t2927: F, t287: F, t2922: F, t275: F) -> (F, F, F) {
    let t11296 = F::cast_from(0.48245938496077605201e2_f64) * t11294 * t2927;
    let t11298 = F::cast_from(1.0_f64) / t2922 / t287;
    let t11299 = t275 * t11298;
    (t11296, t11298, t11299)
}
