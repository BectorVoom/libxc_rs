//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1243/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1243(t25721: f64, t6743: f64, t210: f64, t23599: f64, t23632: f64, t1958: f64, t43637: f64, t38: f64, t9287: f64, t835: f64, t39063: f64, t6489: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t83240 = t6743 * t25721;
    let t83244 = t23599 * t210;
    let t83245 = t83244 * t23632;
    let t83479 = t1958 * t43637;
    let t83796 = t38 * t9287;
    let t83803 = 1232.0_f64 / 27.0_f64 * t835;
    let t83830 = t39063 * t6489;
    (t83240, t83245, t83479, t83796, t83803, t83830)
}
