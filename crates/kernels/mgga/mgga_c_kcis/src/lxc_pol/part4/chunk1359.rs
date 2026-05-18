//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1359/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1359<F: Float>(t15936: F, t6028: F, t6027: F, t17333: F, t4261: F, t17428: F, t17431: F, t17434: F, t17437: F, t17439: F, t17441: F, t17444: F, t17447: F, t17451: F, t17455: F, t17458: F, t17461: F, t17465: F, t17468: F, t17472: F, t17475: F) -> (F, F, F) {
    let t17477 = t6028 * t15936;
    let t17478 = t6027 * t17477;
    let t17480 = t4261 * t17333;
    let t17481 = t6027 * t17480;
    let t17483 = t17428 / F::new(108.0) - t17431 / F::new(12.0) + t17434 / F::new(36.0) - t17437 / F::new(96.0) + t17439 / F::new(128.0) - t17441 / F::new(96.0) - t17444 / F::new(288.0) + t17447 / F::new(96.0) - t17451 / F::new(8.0) + t17455 / F::new(288.0) + t17458 / F::new(192.0) - t17461 / F::new(24.0) + F::new(3.0) / F::new(128.0) * t17465 + t17468 / F::new(192.0) + t17472 / F::new(864.0) + F::new(2.0) / F::new(9.0) * t17475 + t17478 / F::new(8.0) + t17481 / F::new(6.0);
    (t17478, t17481, t17483)
}
