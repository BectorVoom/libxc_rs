//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 948/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk948<F: Float>(t15936: F, t6028: F, t6027: F, t17333: F, t4261: F, t17428: F, t17431: F, t17434: F, t17437: F, t17439: F, t17441: F, t17444: F, t17447: F, t17451: F, t17455: F, t17458: F, t17461: F, t17465: F, t17468: F, t17472: F, t17475: F) -> (F, F, F, F) {
    let t17477 = t6028 * t15936;
    let t17478 = t6027 * t17477;
    let t17480 = t4261 * t17333;
    let t17481 = t6027 * t17480;
    let t17483 = t17428 / F::cast_from(108.0_f64) - t17431 / F::cast_from(12.0_f64) + t17434 / F::cast_from(36.0_f64) - t17437 / F::cast_from(96.0_f64) + t17439 / F::cast_from(128.0_f64) - t17441 / F::cast_from(96.0_f64) - t17444 / F::cast_from(288.0_f64) + t17447 / F::cast_from(96.0_f64) - t17451 / F::cast_from(8.0_f64) + t17455 / F::cast_from(288.0_f64) + t17458 / F::cast_from(192.0_f64) - t17461 / F::cast_from(24.0_f64) + F::cast_from(3.0_f64) / F::cast_from(128.0_f64) * t17465 + t17468 / F::cast_from(192.0_f64) + t17472 / F::cast_from(864.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t17475 + t17478 / F::cast_from(8.0_f64) + t17481 / F::cast_from(6.0_f64);
    (t17477, t17478, t17481, t17483)
}
