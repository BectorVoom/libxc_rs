//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1984/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1984(t87463: f64, t87477: f64, t87487: f64, t81957: f64, t81964: f64, t84932: f64, t87458: f64, t87466: f64, t87469: f64, t87472: f64, t87475: f64, t87481: f64, t87485: f64, t87491: f64, t87495: f64, t87498: f64, t87502: f64, t87507: f64) -> f64 {
    let t92705 = 7.0_f64 / 12.0_f64 * t87463;
    let t92710 = 0.33913115119077928316e-1_f64 * t87477;
    let t92713 = 0.56521858531796547194e-2_f64 * t87487;
    let t92719 = -0.48447307312968469024e-2_f64 * t87458 - t84932 - 7.0_f64 / 24.0_f64 * t81957 - 0.11869590291677274911e0_f64 * t81964 - t92705 + t87466 / 4.0_f64 + t87469 / 8.0_f64 - 0.40372756094140390853e-3_f64 * t87472 - 0.80745512188280781706e-3_f64 * t87475 - t92710 - 0.40372756094140390853e-3_f64 * t87481 + 0.24223653656484234512e-2_f64 * t87485 + t92713 + 0.24223653656484234512e-2_f64 * t87491 - 0.40372756094140390853e-3_f64 * t87495 + 0.16149102437656156341e-2_f64 * t87498 + 0.24223653656484234512e-2_f64 * t87502 - 0.96894614625936938048e-2_f64 * t87507;
    t92719
}
