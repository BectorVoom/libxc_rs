//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 860/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk860<F: Float>(t41749: F, t6717: F, t6914: F, t40449: F, t40452: F, t10608: F, t9272: F, t9278: F, t1445: F, t26809: F, t3085: F, t4527: F) -> (F, F, F, F, F) {
    let t42315 = F::new(0.12423108009070322895e3) * t6914 * t6717 * t41749;
    let t42340 = F::new(0.63904876589867916127e-1) * t40449;
    let t42341 = F::new(0.31952438294933958063e0) * t40452;
    let t42349 = t9272 * t10608 * t9278;
    let t42350 = F::new(0.11502877786176224903e1) * t42349;
    let t42354 = F::new(0.27606906686822939767e2) * t4527 * t1445 * t26809 * t3085;
    (t42315, t42340, t42341, t42350, t42354)
}
