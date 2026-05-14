//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1283/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1283<F: Float>(t12: F, t154: F, t2048: F, t276: F, t9161: F, t1429: F, t1541: F, t1643: F, t1646: F, t17361: F, t1837: F, t20741: F, t23948: F, t2732: F, t3363: F, t3366: F, t439: F, t4803: F, t5528: F, t7337: F, t78: F, t8729: F, t9150: F, t9155: F, zeta_threshold: F) -> (F, F) {
    let t84 = t12 <= zeta_threshold;
    let t25290 = t276 * t154 * t2048 * t9161;
    let t25315 = piecewise3(t84, 0.0, 280.0 / 81.0 * t17361 * t3363 * t1643 - 224.0 / 27.0 * t7337 * t23948 - 28.0 / 27.0 * t9150 * t1646 + 32.0 / 9.0 * t1837 * t78 * t1541 + 16.0 / 9.0 * t2732 * t1429 - 16.0 / 3.0 * t2732 * t4803 - 28.0 / 27.0 * t5528 * t3366 * t1643 + 8.0 / 9.0 * t1837 * t8729 * t439 + 4.0 / 9.0 * t9155 * t1646 - t20741);
    (t25290, t25315)
}
