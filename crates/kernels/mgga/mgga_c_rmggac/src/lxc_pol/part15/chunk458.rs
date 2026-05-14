//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 458/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk458<F: Float>(t60: F, t1802: F, t4408: F, t1805: F, t990: F, t1383: F, t284: F, t441: F, t5873: F, t815: F, t6053: F, zeta_threshold: F) -> (F,) {
    let t61 = t60 <= zeta_threshold;
    let t6054 = t4408 * t1802;
    let t6059 = t990 * t1805;
    let t6065 = piecewise3(t61, 0.0, 8.0 / 27.0 * t6054 * t284 + 8.0 / 9.0 * t1383 * t815 - 2.0 / 9.0 * t6059 * t284 + 2.0 / 3.0 * t441 * t5873);
    let t6067 = t6053 / 2.0 + t6065 / 2.0;
    (t6067,)
}
