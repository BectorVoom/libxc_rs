//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 523/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk523<F: Float>(t221: F, t2485: F, t837: F, t2484: F, t737: F) -> (F, F, F, F) {
    let t2487 = t2485 * t221 * t837;
    let t2488 = t2484 * t2487;
    let t2490 = t737 * t737;
    let t2491 = F::new(1.0) / t2490;
    (t2487, t2488, t2490, t2491)
}
