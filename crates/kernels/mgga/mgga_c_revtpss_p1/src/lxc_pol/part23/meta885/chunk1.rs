//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2799/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2799<F: Float>(t21969: F, t566: F, t1450: F, t22461: F, t116: F, t21813: F, t21830: F, t625: F, t2289: F, t5916: F, t21877: F, t1507: F, t2357: F) -> (F, F, F, F, F, F, F) {
    let t75379 = t566 * t21969;
    let t75389 = t22461 * t1450;
    let t75439 = t21813 * t116;
    let t75526 = t625 * t21830;
    let t75540 = t2289 * t5916;
    let t75542 = t625 * t21877;
    let t75625 = t1507 * t2357;
    (t75379, t75389, t75439, t75526, t75540, t75542, t75625)
}
