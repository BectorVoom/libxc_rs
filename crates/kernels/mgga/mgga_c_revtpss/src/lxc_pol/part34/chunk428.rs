//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 428/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk428<F: Float>(t251: F, t785: F, t780: F, t2439: F, t211: F, t784: F) -> (F, F, F, F) {
    let t2440 = t785 * t251;
    let t2441 = t2440 * t780;
    let t2443 = 0.65049603595885220126e-3 * t2439 * t2441;
    let t2452 = 1.0 / t784 / t211;
    (t2440, t2441, t2443, t2452)
}
