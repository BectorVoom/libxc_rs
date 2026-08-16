//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 861/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk861(t3781: f64, t3786: f64, t850: f64, t860: f64, t9144: f64, t1109: f64, t1134: f64, t858: f64, t3065: f64, t8978: f64, t11414: f64, t9016: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13518 = t850 * t3781 * t3786;
    let t13520 = t13518 * t860 / 48.0_f64;
    let t13522 = 35.0_f64 / 144.0_f64 * t9144;
    let t13523 = t1134 * t1109;
    let t13524 = t858 * t13523;
    let t13525 = t3065 * t13524;
    let t13527 = t8978 * t13525 / 32.0_f64;
    let t13529 = t9016 * t11414 / 8.0_f64;
    (t13518, t13520, t13522, t13523, t13524, t13525, t13527, t13529)
}
