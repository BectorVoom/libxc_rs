//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1315/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1315<F: Float>(t3128: F, t54359: F, t14570: F, t9111: F, t11824: F, t14015: F, t11466: F, t14011: F, t11578: F, t14498: F, t14535: F, t3113: F) -> (F, F, F, F, F, F) {
    let t57044 = t3128 * t54359;
    let t57046 = t9111 * t14570;
    let t57048 = t14015 * t11824;
    let t57050 = t14011 * t11466;
    let t57052 = t14498 * t11578;
    let t57054 = t3113 * t14535;
    (t57044, t57046, t57048, t57050, t57052, t57054)
}
