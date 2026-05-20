//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1119/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1119<F: Float>(t2681: F, t7269: F, t820: F, t240: F, t25981: F, t2453: F, t4086: F, t64: F, t2018: F, t40688: F, t46808: F, t7256: F, t9784: F) -> (F, F, F, F, F) {
    let t94545 = t820 * t7269 * t2681;
    let t94550 = t25981 * t240;
    let t94564 = t2453 * t4086 * t64;
    let t94568 = t40688 * t2018 * t46808;
    let t94570 = t9784 * t7256;
    (t94545, t94550, t94564, t94568, t94570)
}
