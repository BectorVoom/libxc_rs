//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 943/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk943<F: Float>(t3492: F, t586: F, t645: F, t2654: F, t5390: F, t3603: F, t735: F, t3342: F, t476: F, t3346: F, t92: F, t3351: F, t478: F) -> (F, F, F, F, F, F) {
    let t10629 = t3492 * t586;
    let t10631 = F::new(8.0) / F::new(45.0) * t10629 * t645;
    let t10633 = F::new(0.2e-20) * t2654 * t5390;
    let t10634 = t3603 * t735;
    let t10636 = t476 * t3342;
    let t10641 = t92 * t3346;
    let t10646 = t478 * t3351;
    (t10631, t10633, t10634, t10636, t10641, t10646)
}
