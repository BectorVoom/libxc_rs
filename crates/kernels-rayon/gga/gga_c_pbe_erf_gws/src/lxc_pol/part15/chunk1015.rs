//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1015/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1015(t3116: f64, t6605: f64, t6603: f64, t343: f64, t8890: f64, t858: f64, t2407: f64, t6672: f64, t2170: f64, t875: f64, t8961: f64, t2168: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9123 = 7.0_f64 / 144.0_f64 * t3116 * t6605;
    let t9124 = 7.0_f64 / 288.0_f64 * t6603;
    let t9125 = t8890 * t343;
    let t9126 = t858 * t9125;
    let t9127 = t2407 * t9126;
    let t9129 = t6672 * t9127 / 24.0_f64;
    let t9131 = t2170 * t8961 * t875;
    let t9133 = t2168 * t9131 / 24.0_f64;
    (t9123, t9124, t9127, t9129, t9131, t9133)
}
