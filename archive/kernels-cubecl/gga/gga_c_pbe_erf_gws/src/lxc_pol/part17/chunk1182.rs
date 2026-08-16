//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1182/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1182<F: Float>(t1193: F, t2100: F, t353: F, t859: F, t13791: F, t2387: F, t2227: F, t14127: F, t2397: F, t1452: F, t331: F, t20154: F, t3067: F, t4007: F, t938: F) -> (F, F, F, F, F, F) {
    let t50881 = t859 * t353 * t1193 * t2100;
    let t50884 = t2387 * t13791;
    let t50891 = t859 * t2227;
    let t50904 = t14127 * t2397;
    let t50906 = t1452 * t331;
    let t50919 = t20154 * t3067 * t4007 * t938;
    (t50881, t50884, t50891, t50904, t50906, t50919)
}
