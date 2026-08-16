//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2263/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2263(t1983: f64, t22578: f64, t7753: f64, t22607: f64, t7756: f64, t531: f64, t7752: f64, t22596: f64, t16153: f64, t24995: f64, t8945: f64, t22574: f64, t25988: f64, t31035: f64) -> (f64, f64, f64, f64, f64) {
    let t91673 = t1983 * t7753 * t22578;
    let t91674 = t22607 * t7756;
    let t91675 = t531 * t7752;
    let t91678 = 6.0_f64 * t1983 * t91675 * t22596;
    let t91681 = 6.0_f64 * t24995 * t8945 * t16153;
    let t91684 = 6.0_f64 * t22574 * t31035 * t25988;
    (t91673, t91674, t91678, t91681, t91684)
}
