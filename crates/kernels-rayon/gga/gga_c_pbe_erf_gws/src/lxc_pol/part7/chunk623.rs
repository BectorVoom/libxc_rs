//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 623/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk623(t1448: f64, t4850: f64, t1438: f64, t461: f64, t4358: f64, t88: f64, t4560: f64, t1332: f64, t408: f64, t36: f64, t4259: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4851 = t4850 * t1448;
    let t4852 = 0.32530742648344572643e-1_f64 * t4851;
    let t4853 = t1438 * t461;
    let t4854 = 96.0_f64 * t4853;
    let t4855 = t4358 * t88;
    let t4856 = 24.0_f64 * t4855;
    let t4857 = t4560 * t88;
    let t4858 = 144.0_f64 * t4857;
    let t4859 = t408 * t1332;
    let t4860 = t4859 * t88;
    let t4861 = 240.0_f64 * t4860;
    let t4862 = t36 * t4259;
    (t4851, t4852, t4853, t4854, t4855, t4856, t4857, t4858, t4859, t4860, t4861, t4862)
}
