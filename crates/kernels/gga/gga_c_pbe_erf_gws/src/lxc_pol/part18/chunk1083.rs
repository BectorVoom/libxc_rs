//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1083/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1083<F: Float>(t51977: F, t745: F, t837: F, t2306: F, t938: F, t4002: F, t4424: F, t1477: F, t274: F, t833: F, t850: F, t851: F, t1172: F, t1198: F, t319: F, t12276: F, t50832: F) -> (F, F, F, F, F, F, F, F) {
    let t51978 = 455.0 / 1296.0 * t51977;
    let t51989 = t745 * t837;
    let t52000 = t2306 * t938;
    let t52020 = t4424 * t4002;
    let t52033 = t274 * t1477;
    let t52036 = t850 * t851 * t52033 * t833;
    let t52774 = t1172 * t319 * t1198;
    let t52789 = 6.0 * t50832 * t12276;
    (t51978, t51989, t52000, t52020, t52033, t52036, t52774, t52789)
}
