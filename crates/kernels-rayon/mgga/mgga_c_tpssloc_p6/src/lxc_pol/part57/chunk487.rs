//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 487/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk487(t1137: f64, t6052: f64, t3359: f64, t6036: f64, t3363: f64, t4721: f64, t5973: f64, t5977: f64, t5981: f64, t449: f64, t1694: f64, t1156: f64) -> (f64, f64, f64, f64, f64) {
    let t6053 = t6052 * t1137;
    let t6056 = t6036 * t3359;
    let t6063 = t3363 - 0.61805555555555555556e-2_f64 * t4721 - 0.61805555555555555555e-2_f64 * t5973 + 0.18541666666666666667e-1_f64 * t5977 + 0.92708333333333333333e-2_f64 * t5981;
    let t6064 = t6063 * t449;
    let t6068 = t1694 * t1694;
    let t6069 = t6068 * t1156;
    (t6053, t6056, t6064, t6068, t6069)
}
