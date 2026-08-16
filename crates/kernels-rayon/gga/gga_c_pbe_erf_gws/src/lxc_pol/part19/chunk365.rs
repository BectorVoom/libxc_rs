//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 365/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk365(t1134: f64, t858: f64, t867: f64, t866: f64, t1105: f64, t886: f64) -> (f64, f64, f64, f64, f64) {
    let t1135 = t858 * t1134;
    let t1136 = t867 * t1135;
    let t1138 = t866 * t1136 / 96.0_f64;
    let t1139 = t858 * t1105;
    let t1140 = t886 * t1139;
    (t1135, t1136, t1138, t1139, t1140)
}
