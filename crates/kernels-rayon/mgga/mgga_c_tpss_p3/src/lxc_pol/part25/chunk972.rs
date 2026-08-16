//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 972/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk972(t13392: f64, t581: f64, t3431: f64, t3455: f64, t2016: f64, t4579: f64, t13335: f64, t60: f64, t1300: f64, t13371: f64, t13374: f64, t13380: f64, t13383: f64, t3456: f64, t3459: f64, t44: f64, t4589: f64, t4592: f64, t4597: f64, t56: f64, t589: f64, t595: f64, t7761: f64) -> f64 {
    let t13393 = t13392 * t581;
    let t13396 = t3455 * t3431;
    let t13399 = t2016 * t4579;
    let t13400 = t13399 * t581;
    let t13403 = t60 * t13335;
    let t13406 = -20.0_f64 / 27.0_f64 * t589 * t4589 - 5.0_f64 / 108.0_f64 * t44 * t13371 + 5.0_f64 / 9.0_f64 * t44 * t13374 - 20.0_f64 / 9.0_f64 * t589 * t4592 + 5.0_f64 / 18.0_f64 * t44 * t13380 + 5.0_f64 / 6.0_f64 * t44 * t13383 - 220.0_f64 / 27.0_f64 * t4597 * t595 - 40.0_f64 / 27.0_f64 * t1300 * t3456 + 40.0_f64 / 9.0_f64 * t1300 * t3459 + 5.0_f64 / 108.0_f64 * t56 * t13393 + 5.0_f64 / 9.0_f64 * t56 * t13396 + 5.0_f64 / 18.0_f64 * t56 * t13400 - 5.0_f64 / 6.0_f64 * t56 * t13403 + t7761;
    t13406
}
