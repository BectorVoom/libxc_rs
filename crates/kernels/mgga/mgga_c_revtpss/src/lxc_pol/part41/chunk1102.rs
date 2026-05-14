//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1102/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1102<F: Float>(t45: F, t57: F, t5819: F, t633: F, t5825: F, t80: F, t18281: F, t4186: F, t4328: F, t606: F, t766: F, t637: F, t83: F, t4335: F, t770: F, t124: F, t800: F, t828: F, t855: F, zeta_threshold: F) -> (F, F, F) {
    let t151 = t45 <= zeta_threshold;
    let t155 = t57 <= zeta_threshold;
    let t18367 = t633 * t5819;
    let t18372 = t80 * t5825;
    let t18378 = piecewise3(t151, 0.0, 8.0 / 27.0 * t18367 * t606 - 4.0 / 9.0 * t4328 * t4186 - 2.0 / 9.0 * t18372 * t606 + 2.0 / 3.0 * t766 * t18281);
    let t18379 = t637 * t5819;
    let t18384 = t83 * t5825;
    let t18390 = piecewise3(t155, 0.0, -8.0 / 27.0 * t18379 * t606 - 4.0 / 9.0 * t4335 * t4186 - 2.0 / 9.0 * t18384 * t606 - 2.0 / 3.0 * t770 * t18281);
    let t18392 = t18378 / 2.0 + t18390 / 2.0;
    let t18393 = t124 * t18392;
    let t18394 = t800 * t18393;
    let t18398 = t855 * t828 * t18392;
    (t18392, t18394, t18398)
}
