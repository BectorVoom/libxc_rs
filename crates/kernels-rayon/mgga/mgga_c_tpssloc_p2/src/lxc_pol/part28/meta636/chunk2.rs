//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2022/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2022(t91281: f64, t91283: f64, t91286: f64, t91290: f64, t91300: f64, t80837: f64, t80843: f64, t80857: f64, t80859: f64, t84520: f64, t91261: f64, t91263: f64, t91268: f64, t91272: f64, t91276: f64, t91279: f64, t91294: f64, t91298: f64) -> f64 {
    let t93710 = 7.0_f64 / 576.0_f64 * t91281;
    let t93711 = 7.0_f64 / 576.0_f64 * t91283;
    let t93712 = 7.0_f64 / 576.0_f64 * t91286;
    let t93715 = 0.33913115119077928316e-1_f64 * t91290;
    let t93718 = 0.11304371706359309439e-1_f64 * t91300;
    let t93719 = -t91261 / 48.0_f64 - 5.0_f64 / 96.0_f64 * t91263 + 0.40372756094140390852e-3_f64 * t80837 - 0.28260929265898273597e-2_f64 * t80843 - t84520 - 0.80745512188280781706e-3_f64 * t91268 + 0.48447307312968469024e-2_f64 * t91272 + 0.24223653656484234512e-2_f64 * t91276 - t91279 / 384.0_f64 + t93710 + t93711 + t93712 - 0.80745512188280781706e-3_f64 * t80857 - 35.0_f64 / 288.0_f64 * t80859 - t93715 + 0.48447307312968469024e-2_f64 * t91294 + 0.24223653656484234512e-2_f64 * t91298 - t93718;
    t93719
}
