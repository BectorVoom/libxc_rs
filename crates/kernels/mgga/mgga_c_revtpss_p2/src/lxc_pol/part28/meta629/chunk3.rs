//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2268/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2268<F: Float>(t101432: F, t101555: F, t97635: F, t98422: F, t98468: F, t98512: F, t98563: F, t98612: F, t1913: F, t7337: F, t1916: F, t26120: F) -> (F, F, F) {
    let t101558 = t97635 + t98422 + t98468 + t98512 + t98563 + t98612 + t101432 + t101555;
    let t101563 = F::new(2.0) * t1913 * t7337;
    let t101568 = F::new(6.0) * t1916 * t26120;
    (t101558, t101563, t101568)
}
