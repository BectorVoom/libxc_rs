//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 837/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk837(t184: f64, t7838: f64, t219: f64, t5400: f64, t5480: f64, t1563: f64, t9: f64, t155: f64, t506: f64, t1503: f64, t522: f64, t524: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7839 = t7838 * t184;
    let t7853 = t5400 * t219;
    let t7877 = t5480 * t219;
    let t8231 = t9 * t1563;
    let t8236 = t155 * t506;
    let t8331 = t1503 * t522 * t524;
    (t7839, t7853, t7877, t8231, t8236, t8331)
}
