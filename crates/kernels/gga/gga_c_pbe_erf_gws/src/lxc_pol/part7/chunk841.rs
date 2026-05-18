//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 841/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk841<F: Float>(t274: F, t745: F, t820: F, t6365: F, t904: F, t875: F, t2306: F, t367: F, t6553: F, t899: F, t2074: F, t254: F, t6: F, t6469: F) -> (F, F, F, F, F, F) {
    let t9332 = t745 * t820 * t274;
    let t9343 = t6365 * t904;
    let t9387 = t875 * t820;
    let t9388 = t2306 * t9387;
    let t9425 = t899 * t6553 * t367;
    let t9465 = t274 * t2074;
    let t9482 = t254 * t6 * t6469;
    (t9332, t9343, t9388, t9425, t9465, t9482)
}
