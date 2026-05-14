//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 592/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk592<F: Float>(t1438: F, t461: F, t4358: F, t88: F, t4560: F, t1332: F, t408: F, t36: F, t4259: F, t713: F, t762: F, t1917: F, t528: F, t220: F, t2735: F, t211: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t4853 = t1438 * t461;
    let t4854 = 96.0 * t4853;
    let t4855 = t4358 * t88;
    let t4856 = 24.0 * t4855;
    let t4857 = t4560 * t88;
    let t4858 = 144.0 * t4857;
    let t4859 = t408 * t1332;
    let t4860 = t4859 * t88;
    let t4861 = 240.0 * t4860;
    let t4862 = t36 * t4259;
    let t4863 = t4862 * t88;
    let t4864 = 120.0 * t4863;
    let t4872 = 0.66490888888888888888e-1 * t762 * t713;
    let t4876 = 0.9973633333333333333e-1 * t528 * t1917;
    let t4908 = t2735 * t220;
    let t4910 = 16.0 / 405.0 * t211 * t4908;
    (t4853, t4854, t4855, t4856, t4857, t4858, t4859, t4860, t4861, t4862, t4863, t4864, t4872, t4876, t4908, t4910)
}
