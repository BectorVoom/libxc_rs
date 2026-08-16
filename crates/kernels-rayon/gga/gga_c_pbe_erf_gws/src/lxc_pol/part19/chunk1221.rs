//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1221/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1221(t3966: f64, t51966: f64, t326: f64, t378: f64, t6594: f64, t745: f64, t837: f64, t2306: f64, t938: f64, t1477: f64, t274: f64, t833: f64, t850: f64, t851: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t51967 = t51966 * t3966;
    let t51977 = t326 * t6594 * t378;
    let t51989 = t745 * t837;
    let t52000 = t2306 * t938;
    let t52033 = t274 * t1477;
    let t52036 = t850 * t851 * t52033 * t833;
    (t51967, t51977, t51989, t52000, t52033, t52036)
}
