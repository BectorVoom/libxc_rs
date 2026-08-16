//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2772/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2772<F: Float>(t22026: F, t46929: F, t808: F, t22135: F, t9744: F, t1413: F, t21969: F, t547: F, t807: F, t221: F, t22274: F, t3978: F, t46716: F) -> (F, F, F, F) {
    let t74362 = t46929 * t808 * t22026;
    let t74364 = t9744 * t22135;
    let t74402 = t807 * t547 * t1413 * t21969;
    let t74419 = t221 * t22274;
    let t74421 = t3978 * t46716 * t74419;
    (t74362, t74364, t74402, t74421)
}
