//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 1041/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk1041(t1692: f64, t2046: f64, t2050: f64, t31: f64, t2604: f64, t8413: f64, t3928: f64, t5187: f64, t645: f64, t4044: f64, t5194: f64, t1356: f64, t2024: f64, t27075: f64, t27136: f64, t289: f64, t36515: f64, t36521: f64, t36528: f64, t36533: f64, t36535: f64, t41647: f64, t41648: f64, t41651: f64, t41654: f64, t41657: f64, t41663: f64, t7703: f64, t884: f64) -> f64 {
    let t41667 = t2046 * t2050 * t1692 * t31;
    let t41668 = 0.43368970657079495312e-4_f64 * t41667;
    let t41669 = t2604 * t8413;
    let t41672 = t3928 * t645 * t5187;
    let t41675 = t4044 * t645 * t5194;
    let t41683 = t41647 - t41648 - 0.66211599834018861286e-4_f64 * t36515 - 0.82764499792523576607e-4_f64 * t36521 - 0.4726e1_f64 * t289 * t41651 + 0.59590439850616975157e-4_f64 * t41654 - t41657 + t36528 + 0.17877131955185092547e-3_f64 * t36533 + 0.59590439850616975158e-4_f64 * t36535 + 0.42564599893297839398e-5_f64 * t41663 + t41668 - 0.5987120850931904282e-1_f64 * t41669 + 0.17961362552795712846e0_f64 * t41672 - 0.35922725105591425692e0_f64 * t41675 - 0.11974241701863808564e0_f64 * t1356 * t7703 * t27075 - 0.11974241701863808564e0_f64 * t884 * t2024 * t27136;
    t41683
}
