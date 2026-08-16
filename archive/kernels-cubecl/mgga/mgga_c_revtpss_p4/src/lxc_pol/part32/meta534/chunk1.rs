//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1842/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1842<F: Float>(t7256: F, t9784: F, t25877: F, t94390: F, t1399: F, t2434: F, t46361: F, t545: F, t1032: F, t9656: F, t25875: F, t25894: F) -> (F, F, F, F, F, F) {
    let t94570 = t9784 * t7256;
    let t94589 = t94390 * t25877;
    let t94633 = t2434 * t1399;
    let t94656 = t46361 * t545;
    let t94667 = t1032 * t9656;
    let t94668 = t94667 * t545;
    let t94669 = t25875 * t94668;
    let t94674 = t25894 * t94668;
    (t94570, t94589, t94633, t94656, t94669, t94674)
}
