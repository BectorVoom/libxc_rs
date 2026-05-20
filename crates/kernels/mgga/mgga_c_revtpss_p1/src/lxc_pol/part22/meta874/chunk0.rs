//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3037/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3037<F: Float>(t10069: F, t14482: F, t15003: F, t41020: F, t14939: F, t213: F, t4470: F, t786: F, t867: F, t2467: F, t14567: F, t2453: F) -> (F, F, F, F, F, F) {
    let t51264 = t10069 * t14482;
    let t51268 = t41020 * t15003;
    let t51272 = t213 * t14939;
    let t51276 = t786 * t4470 * t867;
    let t51277 = t51276 * t2467;
    let t51297 = t2453 * t14567;
    (t51264, t51268, t51272, t51276, t51277, t51297)
}
