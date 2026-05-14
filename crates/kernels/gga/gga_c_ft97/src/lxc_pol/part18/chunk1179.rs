//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1179/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1179<F: Float>(t25754: F, t420: F, t49004: F, t22602: F, t6437: F, t5598: F, t6445: F, t92433: F, t11204: F, t35: F, t373: F, t3056: F, t5537: F, t5546: F, t5555: F, t938: F) -> (F, F, F, F, F, F) {
    let t101098 = t25754 * t420 * t49004;
    let t101107 = t22602 * t6437;
    let t101139 = 0.68099848938271604939e-1 * t5598 * t92433 * t6445;
    let t101145 = t373 * t11204 * t35;
    let t101150 = t5537 * t5546 * t3056;
    let t101161 = t5555 * t938;
    (t101098, t101107, t101139, t101145, t101150, t101161)
}
