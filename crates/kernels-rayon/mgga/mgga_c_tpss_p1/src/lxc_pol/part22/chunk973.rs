//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 973/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk973(t10416: f64, t70: f64, t3426: f64, t602: f64, t10353: f64, t36: f64, t3432: f64, t1290: f64, t2025: f64, t10409: f64, t10413: f64, t1314: f64, t2026: f64, t3427: f64, t3433: f64, t3436: f64, t3483: f64, t603: f64, t616: f64, t71: f64, t85: f64) -> (f64, f64) {
    let t10417 = t10416 * t70;
    let t10420 = t3426 * t602;
    let t10425 = t36 * t10353;
    let t10426 = t10425 * t70;
    let t10429 = t3432 * t602;
    let t10434 = t1290 * t2025;
    let t10439 = t2026 * t1314 / 24.0_f64 + t603 * t3483 / 12.0_f64 + t71 * t10409 / 24.0_f64 - t10413 * t85 / 12.0_f64 - t10417 * t85 / 6.0_f64 - t10420 * t85 / 6.0_f64 - t3427 * t616 / 6.0_f64 - t10426 * t85 / 12.0_f64 - t10429 * t85 / 6.0_f64 - t3433 * t616 / 6.0_f64 - t10434 * t85 / 12.0_f64 - t3436 * t616 / 6.0_f64;
    (t10425, t10439)
}
