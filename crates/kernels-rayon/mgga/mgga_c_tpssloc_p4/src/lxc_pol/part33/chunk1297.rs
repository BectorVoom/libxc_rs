//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1297/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1297(t28525: f64, t344: f64, t6740: f64, t5904: f64, t6764: f64, t1933: f64, t23479: f64, t99665: f64, t1015: f64, t23472: f64, t28586: f64, t17615: f64, t6717: f64) -> (f64, f64, f64, f64, f64) {
    let t99720 = t6740 * t28525 * t344;
    let t99731 = t5904 * t6764;
    let t99774 = t1933 * t99665 * t23479;
    let t99779 = t23472 * t1015 * t28586;
    let t99785 = t6717 * t17615;
    (t99720, t99731, t99774, t99779, t99785)
}
