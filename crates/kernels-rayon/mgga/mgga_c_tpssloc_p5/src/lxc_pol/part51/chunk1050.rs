//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1050/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1050(t26043: f64, t67: f64, t1864: f64, t6509: f64, t7441: f64, t12571: f64, t6489: f64, t1860: f64, t1865: f64, t22544: f64, t22549: f64, t22551: f64, t26009: f64, t26013: f64, t26016: f64, t26021: f64, t26025: f64, t26028: f64, t6486: f64, t6492: f64, t6506: f64, t6510: f64, t7428: f64, t7442: f64, t7446: f64) -> f64 {
    let t26044 = t26043 * t67;
    let t26045 = t26044 * t1864;
    let t26048 = t7441 * t6509;
    let t26051 = t12571 * t6489;
    let t26054 = -5.0_f64 * t22544 * t26009 - 5.0_f64 / 3.0_f64 * t22549 * t26013 - 5.0_f64 / 3.0_f64 * t26016 * t22551 - t6486 * t7446 / 6.0_f64 - t1860 * t26021 / 6.0_f64 - t1860 * t26025 / 6.0_f64 - t26028 * t1865 / 6.0_f64 - t7428 * t6506 / 6.0_f64 - t7428 * t6510 / 6.0_f64 - t6486 * t7442 / 6.0_f64 - t1860 * t26045 / 6.0_f64 - t1860 * t26048 / 6.0_f64 + 5.0_f64 / 6.0_f64 * t26051 * t6492;
    t26054
}
