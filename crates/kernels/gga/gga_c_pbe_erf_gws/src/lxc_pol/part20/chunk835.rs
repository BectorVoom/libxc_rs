//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 835/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk835<F: Float>(t561: F, t7959: F, t2575: F, t4934: F, t1620: F, t2826: F, t583: F, t1076: F, t1365: F, t153: F, t2513: F, t414: F) -> (F, F, F, F, F) {
    let t7960 = t561 * t7959;
    let t7966 = t4934 * t2575;
    let t7968 = F::new(32.0) / F::new(135.0) * t1620 * t7966;
    let t7970 = F::new(8.0) / F::new(45.0) * t2826 * t583;
    let t7981 = t153 * t1365 * t1076;
    let t7983 = t414 * t2513;
    (t7960, t7968, t7970, t7981, t7983)
}
