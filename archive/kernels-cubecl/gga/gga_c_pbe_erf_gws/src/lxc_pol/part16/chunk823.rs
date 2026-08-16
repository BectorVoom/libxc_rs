//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 823/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk823<F: Float>(t2118: F, t6670: F, t822: F, t6638: F, t858: F, t3065: F, t2263: F, t358: F, t356: F, t2252: F, t2157: F, t6395: F) -> (F, F, F, F, F, F, F) {
    let t6677 = t2118 * t6670;
    let t6678 = t822 * t6677;
    let t6679 = t858 * t6638;
    let t6680 = t3065 * t6679;
    let t6683 = t358 * t2263;
    let t6684 = t356 * t6683;
    let t6685 = t6684 * t2252;
    let t6686 = t6395 * t2157;
    (t6677, t6678, t6680, t6683, t6684, t6685, t6686)
}
