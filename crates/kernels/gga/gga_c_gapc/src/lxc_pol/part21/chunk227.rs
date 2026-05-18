//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 227/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk227<F: Float>(t260: F, t786: F, t154: F, t276: F, t299: F, t311: F, t751: F, t837: F, t841: F, t845: F, t869: F, t871: F) -> (F, F) {
    let t872 = t260 * t786;
    let t875 = F::new(0.14341111111111111111e-1) * t154 * t837 * t276 + F::new(0.21511666666666666667e-1) * t154 * t841 * t276 - F::new(0.21511666666666666667e-1) * t154 * t299 * t845 - t869 * t260 + t871 * t872 - t311 * t751;
    (t872, t875)
}
