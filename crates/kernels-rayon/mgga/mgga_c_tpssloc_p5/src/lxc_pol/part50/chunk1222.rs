//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1222/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1222(t24987: f64, t8494: f64, t114360: f64, t25989: f64, t26142: f64, t8526: f64, t22461: f64, t7468: f64, t111: f64, t33123: f64, t671: f64, t8312: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t119796 = t24987 * t8494;
    let t119799 = t114360 * t25989;
    let t119810 = 4.0_f64 * t8526 * t26142;
    let t119811 = t22461 * t7468;
    let t119815 = t33123 * t111;
    let t119820 = t8312 * t671;
    (t119796, t119799, t119810, t119811, t119815, t119820)
}
