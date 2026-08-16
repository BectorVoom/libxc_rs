//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 695/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk695(t1863: f64, t22550: f64, t6489: f64, t9231: f64, t1860: f64, t1865: f64, t22490: f64, t22493: f64, t22513: f64, t22516: f64, t22519: f64, t22523: f64, t22527: f64, t22531: f64, t22534: f64, t22537: f64, t22544: f64, t22546: f64, t22549: f64, t6486: f64, t6490: f64, t6492: f64, t6495: f64, t6506: f64, t6510: f64) -> f64 {
    let t22551 = t1863 * t22550;
    let t22554 = t9231 * t6489;
    let t22557 = -t1860 * t22490 / 6.0_f64 - t22493 * t1865 / 6.0_f64 - t6486 * t6506 / 3.0_f64 - t6486 * t6510 / 3.0_f64 - t1860 * t22513 / 6.0_f64 - t1860 * t22516 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t22519 * t1865 + 5.0_f64 / 3.0_f64 * t22523 * t6492 + 5.0_f64 / 3.0_f64 * t6490 * t22527 + 5.0_f64 / 6.0_f64 * t6490 * t22531 + t22534 * t1865 / 3.0_f64 + t22537 * t1865 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t6495 * t6506 + 2.0_f64 / 3.0_f64 * t6495 * t6510 - 5.0_f64 * t22544 * t22546 - 10.0_f64 / 3.0_f64 * t22549 * t22551 + 5.0_f64 / 3.0_f64 * t22554 * t6492;
    t22557
}
