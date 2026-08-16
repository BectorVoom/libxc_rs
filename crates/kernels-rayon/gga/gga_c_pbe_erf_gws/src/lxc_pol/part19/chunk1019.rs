//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1019/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1019(t1161: f64, t3306: f64, t2409: f64, t3067: f64, t1105: f64, t1134: f64, t858: f64, t2407: f64, t6672: f64, t9016: f64, t9127: f64, t1114: f64, t8987: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11407 = t1161 * t3306;
    let t11409 = t2409 * t3067 * t11407;
    let t11412 = t1134 * t1105;
    let t11413 = t858 * t11412;
    let t11414 = t2407 * t11413;
    let t11416 = t6672 * t11414 / 24.0_f64;
    let t11418 = t9016 * t9127 / 24.0_f64;
    let t11419 = t1114 * t8987;
    (t11407, t11409, t11414, t11416, t11418, t11419)
}
