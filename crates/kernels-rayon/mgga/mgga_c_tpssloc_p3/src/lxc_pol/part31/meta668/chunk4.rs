//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1969/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1969(t84896: f64, t84897: f64, t87304: f64, t87306: f64, t92626: f64, t92627: f64, t92630: f64, t98715: f64, t98717: f64, t98719: f64, t98721: f64, t98723: f64, t98725: f64, t98728: f64, t98731: f64, t98733: f64, t98736: f64, t98738: f64) -> f64 {
    let t101439 = -t84896 - t84897 + t92626 + t92627 - t92630 + 5.0_f64 / 96.0_f64 * t98715 - 5.0_f64 / 32.0_f64 * t98717 + 5.0_f64 / 96.0_f64 * t98719 + 5.0_f64 / 192.0_f64 * t98721 - t98723 / 768.0_f64 + 0.28260929265898273597e-2_f64 * t98725 - 0.96894614625936938048e-2_f64 * t98728 + 0.48447307312968469024e-2_f64 * t98731 + 7.0_f64 / 288.0_f64 * t98733 - 35.0_f64 / 54.0_f64 * t87304 - 0.27130492095262342653e0_f64 * t87306 + 7.0_f64 / 1152.0_f64 * t98736 + 7.0_f64 / 576.0_f64 * t98738;
    t101439
}
