//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 748/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk748(t1438: f64, t461: f64, t4358: f64, t88: f64, t4560: f64, t1332: f64, t408: f64, t36: f64, t4259: f64, t713: f64, t762: f64, t1597: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4853 = t1438 * t461;
    let t4855 = t4358 * t88;
    let t4856 = 24.0_f64 * t4855;
    let t4857 = t4560 * t88;
    let t4859 = t408 * t1332;
    let t4860 = t4859 * t88;
    let t4862 = t36 * t4259;
    let t4863 = t4862 * t88;
    let t4864 = 120.0_f64 * t4863;
    let t4872 = 0.66490888888888888888e-1_f64 * t762 * t713;
    let t4873 = t1597 * t713;
    (t4853, t4856, t4857, t4860, t4864, t4872, t4873)
}
