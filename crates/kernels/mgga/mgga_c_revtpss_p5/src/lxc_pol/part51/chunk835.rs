//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 835/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk835<F: Float>(t1580: F, t7014: F, t689: F, t27279: F, t7058: F, t72: F, t7769: F, t686: F, t25375: F, t25387: F, t1559: F, t886: F) -> (F, F, F, F, F) {
    let t27334 = t7014 * t1580;
    let t27335 = t689 * t27334;
    let t27338 = t7058 * t27279;
    let t27340 = t7769 * t72;
    let t27341 = t27340 * t686;
    let t27342 = t25375 * t27341;
    let t27344 = t25387 * t27341;
    let t27349 = t1559 * t886;
    (t27335, t27338, t27342, t27344, t27349)
}
