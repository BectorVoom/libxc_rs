//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 1041/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk1041<F: Float>(t1692: F, t2046: F, t2050: F, t31: F, t2604: F, t8413: F, t3928: F, t5187: F, t645: F, t4044: F, t5194: F, t1356: F, t2024: F, t27075: F, t27136: F, t289: F, t36515: F, t36521: F, t36528: F, t36533: F, t36535: F, t41647: F, t41648: F, t41651: F, t41654: F, t41657: F, t41663: F, t7703: F, t884: F) -> F {
    let t41667 = t2046 * t2050 * t1692 * t31;
    let t41668 = F::cast_from(0.43368970657079495312e-4_f64) * t41667;
    let t41669 = t2604 * t8413;
    let t41672 = t3928 * t645 * t5187;
    let t41675 = t4044 * t645 * t5194;
    let t41683 = t41647 - t41648 - F::cast_from(0.66211599834018861286e-4_f64) * t36515 - F::cast_from(0.82764499792523576607e-4_f64) * t36521 - F::new(0.4726e1) * t289 * t41651 + F::cast_from(0.59590439850616975157e-4_f64) * t41654 - t41657 + t36528 + F::cast_from(0.17877131955185092547e-3_f64) * t36533 + F::cast_from(0.59590439850616975158e-4_f64) * t36535 + F::cast_from(0.42564599893297839398e-5_f64) * t41663 + t41668 - F::cast_from(0.5987120850931904282e-1_f64) * t41669 + F::cast_from(0.17961362552795712846e0_f64) * t41672 - F::cast_from(0.35922725105591425692e0_f64) * t41675 - F::cast_from(0.11974241701863808564e0_f64) * t1356 * t7703 * t27075 - F::cast_from(0.11974241701863808564e0_f64) * t884 * t2024 * t27136;
    t41683
}
