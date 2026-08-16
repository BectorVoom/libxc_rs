//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1191/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1191<F: Float>(t2276: F, t51213: F, t2281: F, t1477: F, t345: F, t56: F, t859: F, t854: F, t2407: F, t810: F, t814: F, t858: F) -> (F, F, F, F, F) {
    let t51214 = t2276 * t51213;
    let t51215 = t51214 * t2281;
    let t51221 = t345 * t1477 * t56 * t859;
    let t51222 = t854 * t51221;
    let t51237 = t2407 * t858 * t814 * t810;
    (t51214, t51215, t51221, t51222, t51237)
}
