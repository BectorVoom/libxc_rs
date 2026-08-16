//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1015/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1015(t18440: f64, t1322: f64, t4619: f64, t470: f64, t4800: f64, t1: f64, t467: f64, t4778: f64, t174: f64, t388: f64, t405: f64, t837: f64) -> (f64, f64, f64, f64, f64) {
    let t18441 = 0.2077890707925103596e3_f64 * t18440;
    let t18442 = t1322 * t4619;
    let t18445 = 0.69263023597503453196e2_f64 * t470 * t4800 * t18442;
    let t18447 = t4778 * t1 * t467;
    let t18448 = 0.73246220147012639764e-3_f64 * t18447;
    let t18452 = 0.22161481481481481481e0_f64 * t174 * t837 * t388 * t405;
    (t18441, t18442, t18445, t18448, t18452)
}
