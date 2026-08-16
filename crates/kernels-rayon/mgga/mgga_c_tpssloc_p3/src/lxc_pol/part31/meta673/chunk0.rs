//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2024/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2024(t84533: f64, t91305: f64, t91312: f64, t91314: f64, t91323: f64, t91346: f64, t93720: f64, t93722: f64, t93731: f64, t93736: f64, t93742: f64, t93743: f64, t93745: f64, t97378: f64, t97380: f64, t97382: f64, t97387: f64, t97389: f64) -> f64 {
    let t102715 = -t93720 + 119.0_f64 / 1728.0_f64 * t91305 + t93722 - 0.21083550404717759668e-2_f64 * t91312 - t91314 + 7.0_f64 / 1152.0_f64 * t97378 - 7.0_f64 / 576.0_f64 * t97380 + t97382 / 384.0_f64 + 0.40372756094140390853e-3_f64 * t91323 + t93731 + 0.24223653656484234512e-2_f64 * t97387 + t97389 / 192.0_f64 - t93736 + 0.6728792682356731809e-4_f64 * t91346 - t84533 - t93742 + t93743 - t93745;
    t102715
}
