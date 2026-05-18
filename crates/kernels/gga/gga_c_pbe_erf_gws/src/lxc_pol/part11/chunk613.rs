//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 613/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk613<F: Float>(t1447: F, t4847: F, t1438: F, t461: F, t4358: F, t88: F, t4560: F, t1332: F, t408: F, t36: F, t4259: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t4848 = t1447 * t4847;
    let t4849 = F::new(0.16265371324172286321e-1) * t4848;
    let t4853 = t1438 * t461;
    let t4854 = F::new(96.0) * t4853;
    let t4855 = t4358 * t88;
    let t4856 = F::new(24.0) * t4855;
    let t4857 = t4560 * t88;
    let t4858 = F::new(144.0) * t4857;
    let t4859 = t408 * t1332;
    let t4860 = t4859 * t88;
    let t4861 = F::new(240.0) * t4860;
    let t4862 = t36 * t4259;
    (t4848, t4849, t4853, t4854, t4855, t4856, t4857, t4858, t4859, t4860, t4861, t4862)
}
