//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1198/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1198(t2531: f64, t9722: f64, t2379: f64, t39483: f64, t40727: f64, t40730: f64, t40732: f64, t40734: f64, t40737: f64, t40739: f64, t40741: f64, t40743: f64, t40746: f64, t40748: f64, t40750: f64, t4314: f64, t9470: f64) -> (f64, f64) {
    let t40754 = t2531 * t9722;
    let t40755 = 0.4155806185363551302e3_f64 * t40754;
    let t40756 = -36.0_f64 * t2379 * t4314 * t9470 + t39483 + t40727 + t40730 - t40732 - t40734 + t40737 - t40739 - t40741 - t40743 + t40746 + t40748 + t40750 + t40755;
    (t40755, t40756)
}
