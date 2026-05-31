//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 763/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk763<F: Float>(t1438: F, t461: F, t4358: F, t88: F, t4560: F, t1332: F, t408: F, t36: F, t4259: F, t713: F, t762: F, t1597: F) -> (F, F, F, F, F, F, F) {
    let t4853 = t1438 * t461;
    let t4855 = t4358 * t88;
    let t4856 = F::cast_from(24.0_f64) * t4855;
    let t4857 = t4560 * t88;
    let t4859 = t408 * t1332;
    let t4860 = t4859 * t88;
    let t4862 = t36 * t4259;
    let t4863 = t4862 * t88;
    let t4864 = F::cast_from(120.0_f64) * t4863;
    let t4872 = F::cast_from(0.66490888888888888888e-1_f64) * t762 * t713;
    let t4873 = t1597 * t713;
    (t4853, t4856, t4857, t4860, t4864, t4872, t4873)
}
