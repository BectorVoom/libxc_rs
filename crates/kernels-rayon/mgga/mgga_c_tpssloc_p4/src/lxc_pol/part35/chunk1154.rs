//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1154/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1154(t23110: f64, t7524: f64, t23185: f64, t234: f64, t6604: f64, t1484: f64, t252: f64) -> (f64, f64, f64, f64) {
    let t25245 = t23110 * t7524;
    let t25246 = t23185 * t25245;
    let t25248 = t6604 * t234;
    let t25249 = t252 * t1484;
    (t25245, t25246, t25248, t25249)
}
