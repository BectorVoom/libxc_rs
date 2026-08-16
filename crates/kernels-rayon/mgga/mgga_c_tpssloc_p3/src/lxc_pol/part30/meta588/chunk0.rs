//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1967/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1967(t5544: f64, t868: f64, t5527: f64, t1484: f64, t4303: f64, t4233: f64, t828: f64, t1388: f64, t6347: f64, t1799: f64, t5356: f64, t1351: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t67123 = t5544 * t868;
    let t67128 = t5527 * t868;
    let t67164 = t1484 * t4303;
    let t67783 = t1484 * t4233;
    let t67793 = t5544 * t828;
    let t74032 = t6347 * t1388;
    let t74060 = t1799 * t5356;
    let t74366 = t6347 * t1351;
    (t67123, t67128, t67164, t67783, t67793, t74032, t74060, t74366)
}
