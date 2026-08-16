//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1983/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1983(t1484: f64, t4233: f64, t5544: f64, t828: f64, t1215: f64, t5398: f64, t1388: f64, t6347: f64, t1799: f64, t5356: f64, t1351: f64, t5286: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t67783 = t1484 * t4233;
    let t67793 = t5544 * t828;
    let t72164 = t5398 * t1215;
    let t74032 = t6347 * t1388;
    let t74060 = t1799 * t5356;
    let t74366 = t6347 * t1351;
    let t74677 = t1799 * t5286;
    (t67783, t67793, t72164, t74032, t74060, t74366, t74677)
}
