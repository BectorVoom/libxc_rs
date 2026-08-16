//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1967/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1967(t84857: f64, t84859: f64, t87213: f64, t92580: f64, t92582: f64, t98647: f64, t98651: f64, t98655: f64, t98659: f64, t98663: f64, t98668: f64, t98672: f64, t98674: f64, t98676: f64, t98678: f64, t98680: f64, t98682: f64, t98685: f64) -> f64 {
    let t101413 = 0.40372756094140390853e-3_f64 * t98647 - t92580 - t84857 + t84859 + 0.24223653656484234512e-2_f64 * t98651 - 0.80745512188280781706e-3_f64 * t98655 - 0.40372756094140390853e-3_f64 * t98659 + 0.24223653656484234512e-2_f64 * t98663 + 0.48447307312968469024e-2_f64 * t98668 + 0.48447307312968469024e-2_f64 * t98672 - 5.0_f64 / 96.0_f64 * t98674 + t98676 / 96.0_f64 - t98678 / 384.0_f64 - t98680 / 768.0_f64 - t98682 / 768.0_f64 - t98685 / 768.0_f64 + t92582 + 0.6728792682356731809e-4_f64 * t87213;
    t101413
}
