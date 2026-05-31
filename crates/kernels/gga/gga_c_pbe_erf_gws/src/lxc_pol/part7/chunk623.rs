//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 623/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk623<F: Float>(t1448: F, t4850: F, t1438: F, t461: F, t4358: F, t88: F, t4560: F, t1332: F, t408: F, t36: F, t4259: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t4851 = t4850 * t1448;
    let t4852 = F::cast_from(0.32530742648344572643e-1_f64) * t4851;
    let t4853 = t1438 * t461;
    let t4854 = F::cast_from(96.0_f64) * t4853;
    let t4855 = t4358 * t88;
    let t4856 = F::cast_from(24.0_f64) * t4855;
    let t4857 = t4560 * t88;
    let t4858 = F::cast_from(144.0_f64) * t4857;
    let t4859 = t408 * t1332;
    let t4860 = t4859 * t88;
    let t4861 = F::cast_from(240.0_f64) * t4860;
    let t4862 = t36 * t4259;
    (t4851, t4852, t4853, t4854, t4855, t4856, t4857, t4858, t4859, t4860, t4861, t4862)
}
