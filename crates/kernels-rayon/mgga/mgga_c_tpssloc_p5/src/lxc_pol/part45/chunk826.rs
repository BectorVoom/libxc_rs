//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 826/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk826(t2047: f64, t2591: f64, t23042: f64, t23044: f64, t23049: f64, t23051: f64, t23054: f64, t23057: f64, t23059: f64, t23063: f64, t23067: f64, t23070: f64, t23073: f64, t23081: f64, t23084: f64, t23087: f64, t23090: f64) -> (f64, f64) {
    let t24200 = t2591 * t2047;
    let t24217 = 7.0_f64 / 576.0_f64 * t23042 - t23044 / 768.0_f64 + t23049 / 384.0_f64 - t23051 / 768.0_f64 - t23054 / 384.0_f64 + t23057 / 8.0_f64 - t23059 / 24.0_f64 + 0.33913115119077928316e-1_f64 * t23063 - 0.24223653656484234512e-2_f64 * t23067 + 7.0_f64 / 36.0_f64 * t23070 + 0.80745512188280781706e-3_f64 * t23073 + 0.16956557559538964158e-1_f64 * t23081 + 0.56521858531796547194e-2_f64 * t23084 - 0.40372756094140390853e-3_f64 * t23087 - 0.40372756094140390853e-3_f64 * t23090;
    (t24200, t24217)
}
