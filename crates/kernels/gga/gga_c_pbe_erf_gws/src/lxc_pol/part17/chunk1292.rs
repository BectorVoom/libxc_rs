//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1292/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1292<F: Float>(t14567: F, t2118: F, t27691: F, t3074: F, t3123: F, t6674: F, t2134: F, t9127: F, t14031: F, t9434: F, t9604: F, t3116: F, t51237: F) -> (F, F, F, F, F, F) {
    let t53994 = t3074 * t2118 * t27691 * t14567;
    let t53996 = t3123 * t6674;
    let t53998 = t2134 * t9127;
    let t54000 = t14031 * t9434;
    let t54002 = t14031 * t9604;
    let t54004 = t3116 * t51237;
    (t53994, t53996, t53998, t54000, t54002, t54004)
}
