//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 204/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk204<F: Float>(t11: F, t583: F, t22: F, t21: F, t3: F) -> (F, F, F, F) {
    let t584 = t11 * t583;
    let t586 = F::cast_from(4.0_f64) * t584 * t22;
    let t587 = t21 * t3;
    let t588 = F::cast_from(1.0_f64) / t587;
    (t584, t586, t587, t588)
}
