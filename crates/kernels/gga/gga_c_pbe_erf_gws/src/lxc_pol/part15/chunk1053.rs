//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1053/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1053<F: Float>(t1477: F, t345: F, t56: F, t859: F, t854: F, t2407: F, t810: F, t814: F, t858: F, t14024: F, t2115: F, t2087: F, t13806: F, t2276: F, t932: F, t2315: F) -> (F, F, F, F, F, F, F) {
    let t51221 = t345 * t1477 * t56 * t859;
    let t51222 = t854 * t51221;
    let t51237 = t2407 * t858 * t814 * t810;
    let t51244 = t2115 * t14024;
    let t51252 = t2087 * t14024;
    let t51255 = t2276 * t13806 * t932;
    let t51256 = t51255 * t2315;
    (t51221, t51222, t51237, t51244, t51252, t51255, t51256)
}
