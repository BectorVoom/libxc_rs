//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 866/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk866<F: Float>(t10208: F, t22589: F, t4263: F, t5915: F, t1504: F, t5895: F, t10227: F, t4269: F, t5823: F, t580: F, t9342: F, t100: F) -> (F, F, F, F, F, F, F) {
    let t22590 = t10208 * t22589;
    let t22593 = t4263 * t5915;
    let t22596 = t5895 * t1504;
    let t22597 = t10227 * t22596;
    let t22600 = t4269 * t5823;
    let t22603 = -t580 - t9342;
    let t22604 = F::new(3.0) * t22603;
    let t22605 = t100 * t22604;
    (t22590, t22593, t22597, t22600, t22603, t22604, t22605)
}
