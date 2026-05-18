//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1146/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1146<F: Float>(t14765: F, t2306: F, t3074: F, t833: F, t2409: F, t9716: F, t3959: F, t3298: F, t3975: F, t3972: F, t9707: F, t3965: F) -> (F, F, F, F, F, F, F, F, F) {
    let t14766 = t2306 * t14765;
    let t14767 = t3074 * t14766;
    let t14768 = t14767 * t833;
    let t14772 = t2409 * t9716;
    let t14773 = t3959 * t14772;
    let t14776 = t3975 * t3298;
    let t14777 = t3972 * t14776;
    let t14781 = t2409 * t9707;
    let t14782 = t3965 * t14781;
    (t14766, t14767, t14768, t14772, t14773, t14776, t14777, t14781, t14782)
}
