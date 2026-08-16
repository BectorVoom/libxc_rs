//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2024/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2024(t91356: f64, t91358: f64, t91364: f64, t91386: f64, t80889: f64, t80915: f64, t84533: f64, t84536: f64, t91354: f64, t91362: f64, t91366: f64, t91370: f64, t91374: f64, t91378: f64, t91381: f64, t91384: f64, t91389: f64, t91391: f64) -> f64 {
    let t93742 = 0.33913115119077928316e-1_f64 * t91356;
    let t93743 = 0.56521858531796547194e-2_f64 * t91358;
    let t93745 = 7.0_f64 / 144.0_f64 * t91364;
    let t93753 = 35.0_f64 / 144.0_f64 * t91386;
    let t93756 = -t84533 - 0.11869590291677274911e0_f64 * t80889 - 0.96894614625936938048e-2_f64 * t91354 - t93742 + t93743 - t91362 / 128.0_f64 - t93745 - t84536 - t91366 / 24.0_f64 - 0.24223653656484234512e-2_f64 * t91370 - 0.40372756094140390853e-3_f64 * t91374 + 0.80745512188280781706e-3_f64 * t91378 + 0.16149102437656156341e-2_f64 * t91381 - 119.0_f64 / 1728.0_f64 * t80915 - t91384 / 768.0_f64 - t93753 + t91389 / 384.0_f64 - t91391 / 768.0_f64;
    t93756
}
