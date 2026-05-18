//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1116/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1116<F: Float>(t2447: F, t333: F, t118: F, t25820: F, t25877: F, t27101: F, t352: F, t36248: F, t36269: F, t36272: F, t41501: F, t41506: F, t43261: F, t43914: F, t43925: F, t44157: F, t5266: F, t794: F, t839: F, t848: F, t8940: F, t9523: F, t9540: F, t9551: F) -> F {
    let t44293 = t2447 * t333;
    let t44320 = F::new(0.23948483403727617128e0) * t5266 * t44293 * t352 - F::new(0.39914139006212695214e-1) * t118 * t43925 + F::new(0.23948483403727617128e0) * t8940 * t44157 * t352 + F::new(0.11974241701863808564e0) * t5266 * t9551 * t848 + F::new(0.71845450211182851384e0) * t25877 * t9523 * t839 + F::new(0.11974241701863808564e0) * t41501 - F::new(0.5987120850931904282e-1) * t41506 - F::new(0.71845450211182851384e0) * t25820 * t43261 - F::new(0.47896966807455234256e0) * t27101 * t43914 - F::new(0.35922725105591425692e0) * t25820 * t9540 * t794 + F::new(0.79828278012425390427e-1) * t36248 - F::new(0.43639458646792546768e0) * t36269 - F::new(0.10909864661698136692e0) * t36272;
    t44320
}
