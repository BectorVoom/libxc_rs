//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1116/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1116(t2447: f64, t333: f64, t118: f64, t25820: f64, t25877: f64, t27101: f64, t352: f64, t36248: f64, t36269: f64, t36272: f64, t41501: f64, t41506: f64, t43261: f64, t43914: f64, t43925: f64, t44157: f64, t5266: f64, t794: f64, t839: f64, t848: f64, t8940: f64, t9523: f64, t9540: f64, t9551: f64) -> f64 {
    let t44293 = t2447 * t333;
    let t44320 = 0.23948483403727617128e0_f64 * t5266 * t44293 * t352 - 0.39914139006212695214e-1_f64 * t118 * t43925 + 0.23948483403727617128e0_f64 * t8940 * t44157 * t352 + 0.11974241701863808564e0_f64 * t5266 * t9551 * t848 + 0.71845450211182851384e0_f64 * t25877 * t9523 * t839 + 0.11974241701863808564e0_f64 * t41501 - 0.5987120850931904282e-1_f64 * t41506 - 0.71845450211182851384e0_f64 * t25820 * t43261 - 0.47896966807455234256e0_f64 * t27101 * t43914 - 0.35922725105591425692e0_f64 * t25820 * t9540 * t794 + 0.79828278012425390427e-1_f64 * t36248 - 0.43639458646792546768e0_f64 * t36269 - 0.10909864661698136692e0_f64 * t36272;
    t44320
}
