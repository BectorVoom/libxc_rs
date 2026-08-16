//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 968/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk968(t10353: f64, t48: f64, t1289: f64, t1985: f64, t7750: f64, t2016: f64, t3431: f64, t581: f64, t1992: f64, t3455: f64, t60: f64, t10340: f64, t10344: f64, t10347: f64, t1294: f64, t1300: f64, t2004: f64, t2017: f64, t2020: f64, t3447: f64, t3450: f64, t44: f64, t56: f64, t589: f64, t7761: f64) -> f64 {
    let t10354 = t48 * t10353;
    let t10362 = t7750 * t1289 * t1985;
    let t10365 = t2016 * t3431;
    let t10366 = t10365 * t581;
    let t10369 = t3455 * t1992;
    let t10372 = t60 * t10353;
    let t10375 = 220.0_f64 / 27.0_f64 * t2004 * t1294 - 40.0_f64 / 27.0_f64 * t589 * t3447 - 40.0_f64 / 9.0_f64 * t589 * t3450 - 5.0_f64 / 108.0_f64 * t44 * t10340 + 5.0_f64 / 9.0_f64 * t44 * t10344 + 5.0_f64 / 18.0_f64 * t44 * t10347 + 5.0_f64 / 6.0_f64 * t44 * t10354 - 20.0_f64 / 27.0_f64 * t1300 * t2017 + 20.0_f64 / 9.0_f64 * t1300 * t2020 + 5.0_f64 / 108.0_f64 * t56 * t10362 + 5.0_f64 / 9.0_f64 * t56 * t10366 + 5.0_f64 / 18.0_f64 * t56 * t10369 - 5.0_f64 / 6.0_f64 * t56 * t10372 + t7761;
    t10375
}
