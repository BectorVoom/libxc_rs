//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1059/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1059(t10980: f64, t11003: f64, t11005: f64, t11006: f64, t14459: f64, t14492: f64, t14495: f64, t14505: f64, t14507: f64, t14517: f64, t14521: f64, t14525: f64, t14528: f64, t14532: f64, t14535: f64, t8616: f64, t8687: f64) -> f64 {
    let t14538 = -t8687 - 4.0_f64 / 27.0_f64 * t8616 - 8.0_f64 / 27.0_f64 * t10980 + t11003 - t11005 + t11006 + 2.0_f64 / 27.0_f64 * t14495 - 10.0_f64 / 27.0_f64 * t14517 + 4.0_f64 / 3.0_f64 * t14459 - 4.0_f64 / 9.0_f64 * t14521 - 2.0_f64 / 9.0_f64 * t14505 - 2.0_f64 * t14525 + 4.0_f64 / 3.0_f64 * t14528 + t14507 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t14532 + 2.0_f64 / 3.0_f64 * t14535 - t14492 / 3.0_f64;
    t14538
}
