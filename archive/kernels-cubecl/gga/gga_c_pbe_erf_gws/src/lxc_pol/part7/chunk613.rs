//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 613/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk613<F: Float>(t4656: F, t4741: F, t60: F, t40: F, t1422: F, t460: F, t1322: F, t4605: F, t4607: F, t470: F, t4499: F, t4503: F, t4506: F, t4513: F, t4539: F, t4542: F, t4548: F, t4603: F, t4652: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t4742 = t4656 + t4741;
    let t4743 = t60 * t4742;
    let t4744 = t40 * t4743;
    let t4745 = t1422 * t460;
    let t4746 = t40 * t4745;
    let t4747 = F::cast_from(3.0_f64) * t4746;
    let t4749 = t4605 * t4607 * t1322;
    let t4750 = t470 * t4749;
    let t4751 = F::cast_from(0.1038945353962551798e3_f64) * t4750;
    let t4752 = t4499 + t4503 - t4506 - t4513 + t4539 + t4542 - t4548 + t4603 + t4744 + t4747 + t4751 + t4652;
    (t4742, t4743, t4744, t4745, t4746, t4747, t4749, t4750, t4751, t4752)
}
