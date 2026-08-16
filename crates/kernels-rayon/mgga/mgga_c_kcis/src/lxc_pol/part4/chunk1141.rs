//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1141/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1141(t2635: f64, t4961: f64, t2894: f64, t1704: f64, t2844: f64, t2630: f64, t9933: f64, t14439: f64, t14442: f64, t14446: f64, t14450: f64, t14455: f64, t14460: f64, t14463: f64, t14467: f64, t14470: f64, t1706: f64, t2867: f64, t2872: f64, t4953: f64, t4968: f64, t991: f64) -> f64 {
    let t14473 = t4961 * t2635;
    let t14474 = t2894 * t14473;
    let t14477 = t1704 * t2844;
    let t14478 = t14477 * t2630;
    let t14479 = t9933 * t14478;
    let t14482 = -11.0_f64 / 108.0_f64 * t2867 * t1706 + t14439 - t14442 - t14446 + t14450 - t2872 * t4953 / 27.0_f64 - 7.0_f64 / 432.0_f64 * t14455 - t2872 * t4968 / 9.0_f64 + t991 * t14460 / 48.0_f64 + t991 * t14463 / 48.0_f64 + t991 * t14467 / 144.0_f64 - t991 * t14470 / 36.0_f64 - t991 * t14474 / 288.0_f64 - t991 * t14479 / 216.0_f64;
    t14482
}
