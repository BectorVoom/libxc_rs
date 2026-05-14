//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 967/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk967<F: Float>(t132: F, t1793: F, t2002: F, t2028: F, t341: F, t3627: F, t3630: F, t461: F, t9017: F, t9354: F, t9357: F, t259: F, t9353: F, zeta_threshold: F) -> (F,) {
    let t133 = t132 <= zeta_threshold;
    let t9367 = piecewise3(t133, 0.0, -8.0 / 27.0 * t9354 * t2028 - 16.0 / 9.0 * t9357 * t9017 + 4.0 / 9.0 * t3627 * t2002 - 8.0 / 3.0 * t341 * t1793 + 8.0 * t3630 * t461);
    let t9369 = (t9353 + t9367) * t259;
    (t9369,)
}
