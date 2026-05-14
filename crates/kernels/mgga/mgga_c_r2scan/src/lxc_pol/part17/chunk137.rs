//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 137/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk137<F: Float>(t446: F, t76: F, t390: F, t393: F, t398: F, t388: F) -> (F, F, F, F, F) {
    let t447 = t76 * t446;
    let t449 = 0.301925e0 * t390;
    let t450 = 0.5501625e-1 * t393;
    let t451 = 0.82785e-1 * t398;
    let t452 = -0.86308333333333333334e0 * t388 - t449 - t450 - t451;
    (t447, t449, t450, t451, t452)
}
