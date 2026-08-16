//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1421/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1421(t22522: f64, t9231: f64, t2240: f64, t22511: f64, t33: f64, t1865: f64, t22513: f64, t22519: f64, t22523: f64, t22527: f64, t22544: f64, t22546: f64, t22549: f64, t22554: f64, t6492: f64, t6495: f64, t6506: f64, t6510: f64, t83734: f64, t83738: f64, t83741: f64, t83745: f64, t83748: f64) -> f64 {
    let t83750 = t9231 * t22522;
    let t83760 = t2240 * t33 * t22511;
    let t83766 = -15.0_f64 * t22544 * t83734 - 5.0_f64 * t22549 * t83738 - 15.0_f64 * t83741 * t22546 - 15.0_f64 * t22544 * t83745 + t83748 * t1865 + 5.0_f64 * t83750 * t6492 + 2.0_f64 * t22519 * t6506 + 5.0_f64 * t22554 * t22527 + 2.0_f64 * t22519 * t6510 + 5.0_f64 / 2.0_f64 * t83760 * t6492 + t6495 * t22513 + 5.0_f64 * t22523 * t22527;
    t83766
}
