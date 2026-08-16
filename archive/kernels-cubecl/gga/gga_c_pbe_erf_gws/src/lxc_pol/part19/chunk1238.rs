//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1238/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1238<F: Float>(t28139: F, t850: F, t3065: F, t3167: F, t3253: F, t51255: F, t14099: F, t863: F, t885: F, t1125: F, t51221: F, t3179: F, t51291: F) -> (F, F, F, F, F, F) {
    let t54079 = t850 * t28139;
    let t54084 = t3065 * t3167;
    let t54087 = t51255 * t3253;
    let t54090 = t863 * t14099 * t885;
    let t54094 = t1125 * t51221;
    let t54101 = t51291 * t3179;
    (t54079, t54084, t54087, t54090, t54094, t54101)
}
