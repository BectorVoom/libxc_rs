//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1065/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1065(t413: f64, t5832: f64, t5833: f64, t481: f64, t784: f64, t799: f64, t5795: f64, t119: f64, t1533: f64, t331: f64, t1513: f64, t1544: f64, t542: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19232 = 0.26116266666666666667e1_f64 * t5832 * t5833 * t413;
    let t19234 = t799 * t784 * t481;
    let t19235 = t5795 * t19234;
    let t19236 = 0.51964888888888888888e1_f64 * t19235;
    let t19238 = t119 * t331 * t1533;
    let t19239 = t1513 * t19238;
    let t19240 = 0.38973666666666666666e1_f64 * t19239;
    let t19241 = t542 * t1544;
    (t19232, t19234, t19236, t19238, t19240, t19241)
}
