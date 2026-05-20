//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1480/1505 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1480<F: Float>(t5891: F, t8311: F, t1513: F, t31429: F, t1509: F, t8315: F, t5915: F, t109: F, t1479: F, t655: F, t31433: F, t31149: F, t5907: F) -> (F, F, F, F, F, F, F) {
    let t31626 = t8311 * t5891;
    let t31629 = t31429 * t1513;
    let t31632 = t1513 * t1509;
    let t31633 = t8315 * t31632;
    let t31636 = t8311 * t5915;
    let t31640 = t655 * t1479 * t109;
    let t31643 = t31433 * t1509;
    let t31646 = t31149 * t5907;
    (t31626, t31629, t31633, t31636, t31640, t31643, t31646)
}
