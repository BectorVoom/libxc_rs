//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 886/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk886<F: Float>(t30: F, t33: F, t1344: F, t22670: F, t22769: F, t5574: F, t5824: F, t9605: F, t1348: F, t22778: F, t22783: F, t5582: F, t6416: F, t9617: F, zeta_threshold: F) -> (F,) {
    let t31 = t30 <= zeta_threshold;
    let t34 = t33 <= zeta_threshold;
    let t22799 = piecewise3(t31, 0.0, 8.0 / 27.0 * t9605 * t22769 - 2.0 / 3.0 * t5574 * t5824 + 2.0 / 3.0 * t1344 * t22670);
    let t22807 = piecewise3(t34, 0.0, 8.0 / 27.0 * t9617 * t22778 - 2.0 / 3.0 * t5582 * t6416 + 2.0 / 3.0 * t1348 * t22783);
    let t22809 = t22799 / 2.0 + t22807 / 2.0;
    (t22809,)
}
