//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1309/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1309<F: Float>(t11554: F, t14015: F, t11764: F, t54119: F, t11560: F, t14007: F, t11526: F, t51421: F, t3123: F, t9127: F, t11548: F, t12015: F, t14031: F) -> (F, F, F, F, F, F, F) {
    let t56960 = t14015 * t11554;
    let t56962 = t54119 * t11764;
    let t56964 = t14007 * t11560;
    let t56966 = t51421 * t11526;
    let t56968 = t3123 * t9127;
    let t56970 = t14007 * t11548;
    let t56972 = t14031 * t12015;
    (t56960, t56962, t56964, t56966, t56968, t56970, t56972)
}
