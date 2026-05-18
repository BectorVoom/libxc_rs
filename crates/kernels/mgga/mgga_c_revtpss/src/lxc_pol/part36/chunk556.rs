//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 556/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk556<F: Float>(t1235: F, t5362: F, t1219: F, t1778: F, t1010: F, t1480: F, t1715: F, t3634: F, t247: F) -> (F, F, F, F) {
    let t5363 = t1235 * t5362;
    let t5366 = t1778 * t1219;
    let t5373 = t1480 * t1010;
    let t5377 = t3634 * t1715;
    let t5378 = t247 * t5377;
    (t5363, t5366, t5373, t5378)
}
