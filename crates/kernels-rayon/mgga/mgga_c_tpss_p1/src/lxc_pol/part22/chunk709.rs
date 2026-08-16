//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 709/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk709(t3455: f64, t581: f64, t3431: f64, t60: f64, t1294: f64, t1300: f64, t2024: f64, t3447: f64, t3450: f64, t44: f64, t56: f64, t589: f64, t595: f64) -> f64 {
    let t3456 = t3455 * t581;
    let t3459 = t60 * t3431;
    let t3462 = -20.0_f64 / 9.0_f64 * t589 * t1294 + 5.0_f64 / 18.0_f64 * t44 * t3447 + 5.0_f64 / 6.0_f64 * t44 * t3450 + 20.0_f64 / 9.0_f64 * t1300 * t595 + 5.0_f64 / 18.0_f64 * t56 * t3456 - 5.0_f64 / 6.0_f64 * t56 * t3459 - t2024;
    t3462
}
