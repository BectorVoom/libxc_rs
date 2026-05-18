//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 438/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk438<F: Float>(t1866: F, t598: F, t186: F, t185: F, t1406: F, t198: F) -> (F, F, F, F, F) {
    let t1867 = t598 * t1866;
    let t1868 = t186 * t1867;
    let t1870 = F::new(2.0) / F::new(15.0) * t185 * t1868;
    let t1871 = t198 * t1406;
    let t1872 = t186 * t1871;
    (t1867, t1868, t1870, t1871, t1872)
}
