//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2207/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2207(t23422: f64, t4603: f64, t14159: f64, t6717: f64, t14137: f64, t6765: f64, t7583: f64, t83138: f64, t23509: f64, t25682: f64, t25644: f64, t82926: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t88335 = t23422 * t4603 / 162.0_f64;
    let t88336 = t6717 * t14159;
    let t88339 = 5.0_f64 / 5184.0_f64 * t6765 * t14137;
    let t88341 = 0.20186378047070195428e-3_f64 * t83138 * t7583;
    let t88342 = t23509 * t25682;
    let t88348 = t82926 * t25644;
    (t88335, t88336, t88339, t88341, t88342, t88348)
}
