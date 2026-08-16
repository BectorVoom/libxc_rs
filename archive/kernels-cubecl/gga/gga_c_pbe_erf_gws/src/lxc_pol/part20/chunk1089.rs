//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1089/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1089<F: Float>(t12169: F, t338: F, t353: F, t1161: F, t8713: F, t4386: F, t3739: F, t6832: F, t3907: F, t845: F, t2503: F, t3083: F) -> (F, F, F, F, F) {
    let t12171 = t338 * t353 * t12169;
    let t12180 = t8713 * t1161;
    let t12181 = t353 * t12180;
    let t12182 = t4386 * t12181;
    let t12187 = t6832 * t3739;
    let t12191 = t338 * t3907 * t845;
    let t12195 = t3083 * t2503;
    (t12171, t12182, t12187, t12191, t12195)
}
