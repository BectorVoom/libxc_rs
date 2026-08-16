//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1979/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1979(t87262: f64, t87270: f64, t87272: f64, t81789: f64, t81795: f64, t81797: f64, t81799: f64, t81808: f64, t81810: f64, t81825: f64, t81836: f64, t84896: f64, t84897: f64, t87274: f64, t87276: f64, t87278: f64, t87280: f64, t87284: f64) -> f64 {
    let t92607 = 7.0_f64 / 576.0_f64 * t87262;
    let t92614 = 7.0_f64 / 144.0_f64 * t87270;
    let t92615 = 7.0_f64 / 576.0_f64 * t87272;
    let t92623 = t92607 - 0.12650130242830655801e-1_f64 * t81789 - 0.28260929265898273597e-2_f64 * t81795 - 0.56521858531796547194e-2_f64 * t81797 + 7.0_f64 / 72.0_f64 * t81799 - 119.0_f64 / 1728.0_f64 * t81808 + 7.0_f64 / 1152.0_f64 * t81810 - t92614 + t92615 + t87274 / 384.0_f64 + t87276 / 192.0_f64 + t87278 / 192.0_f64 + t87280 / 192.0_f64 + 7.0_f64 / 576.0_f64 * t81825 - 0.33913115119077928316e-1_f64 * t81836 - t84896 - t84897 - t87284 / 48.0_f64;
    t92623
}
