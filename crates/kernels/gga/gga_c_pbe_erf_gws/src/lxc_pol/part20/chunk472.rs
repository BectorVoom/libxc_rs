//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 472/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk472<F: Float>(t274: F, t343: F, t874: F, t359: F, t362: F, t366: F, t899: F, t745: F, t823: F) -> (F, F, F, F, F, F) {
    let t2257 = t274 * t874 * t343;
    let t2262 = t359 * t359;
    let t2263 = 1.0 / t2262;
    let t2264 = t2263 * t362;
    let t2266 = t899 * t2264 * t366;
    let t2271 = t823 * t745;
    (t2257, t2262, t2263, t2264, t2266, t2271)
}
