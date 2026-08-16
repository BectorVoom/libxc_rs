//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1077/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1077(t20085: f64, t2095: f64, t24432: f64, t28830: f64, t23957: f64, t28826: f64, t26231: f64, t26246: f64, t26251: f64, t26255: f64, t26266: f64, t26268: f64, t28058: f64, t28061: f64, t28063: f64, t28065: f64, t28068: f64, t28070: f64, t28074: f64, t28078: f64, t28080: f64) -> (f64, f64, f64, f64) {
    let t29243 = t2095 * t20085;
    let t29247 = t24432 * t28830;
    let t29252 = t23957 * t28826;
    let t29274 = 7.0_f64 / 576.0_f64 * t26231 + 0.13457585364713463618e-3_f64 * t26246 - 7.0_f64 / 576.0_f64 * t26251 + 0.80745512188280781706e-3_f64 * t28058 - 0.40372756094140390853e-3_f64 * t28061 - t28063 / 768.0_f64 - t28065 / 384.0_f64 - 0.40372756094140390853e-3_f64 * t28068 + 7.0_f64 / 144.0_f64 * t26255 + t28070 / 8.0_f64 + 0.16956557559538964158e-1_f64 * t28074 - 0.24223653656484234512e-2_f64 * t28078 - t28080 / 24.0_f64 + 7.0_f64 / 36.0_f64 * t26266 + 0.33913115119077928316e-1_f64 * t26268;
    (t29243, t29247, t29252, t29274)
}
