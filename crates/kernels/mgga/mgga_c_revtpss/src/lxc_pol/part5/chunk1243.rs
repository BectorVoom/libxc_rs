//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1243/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1243<F: Float>(t21768: F, t38: F, t10389: F, t5819: F, t2299: F, t5825: F, t10398: F, t2306: F, t18281: F, t4186: F, t4227: F, t4232: F, t606: F, t633: F, t637: F, t77: F) -> (F, F) {
    let t21769 = t38 * t21768;
    let t21784 = t10389 * t5819;
    let t21789 = t2299 * t5825;
    let t21794 = t10398 * t5819;
    let t21799 = t2306 * t5825;
    let t21804 = -280.0 / 27.0 * t21784 * t606 + 56.0 / 9.0 * t4227 * t4186 + 28.0 / 9.0 * t21789 * t606 - 4.0 / 3.0 * t633 * t18281 + 280.0 / 27.0 * t21794 * t606 + 56.0 / 9.0 * t4232 * t4186 + 28.0 / 9.0 * t21799 * t606 + 4.0 / 3.0 * t637 * t18281;
    let t21805 = t77 * t21804;
    (t21769, t21805)
}
