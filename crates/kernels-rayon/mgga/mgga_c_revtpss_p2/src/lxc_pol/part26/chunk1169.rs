//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1169/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1169(t25383: f64, t26475: f64, t26511: f64, t7067: f64, t7415: f64, t93126: f64, t95538: f64, t95542: f64, t95543: f64, t95548: f64, t95551: f64, t95553: f64, t95556: f64, t95562: f64, t95567: f64, t95569: f64, t95572: f64) -> f64 {
    let t95574 = -0.15421710918628844643e0_f64 * t95538 - t95542 - 0.38554277296572111609e-1_f64 * t95543 - t95548 - 0.13010442282307799193e1_f64 * t7067 * t26475 - 0.28912093960683998208e-1_f64 * t95551 - 0.86736281882051994623e-1_f64 * t95553 + 0.16463622957338778996e-1_f64 * t95556 - 0.26020884564615598386e1_f64 * t25383 * t26511 - 0.19514881078765566038e-2_f64 * t95562 + 0.26020884564615598386e1_f64 * t93126 * t7415 + t95567 + t95569 - 0.43368140941025997312e-1_f64 * t95572;
    t95574
}
