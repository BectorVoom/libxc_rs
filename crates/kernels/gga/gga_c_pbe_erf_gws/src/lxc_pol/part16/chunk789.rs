//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 789/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk789<F: Float>(t2722: F, t626: F, t422: F, t1815: F, t639: F, t5357: F, t561: F, t213: F, t174: F, t838: F, t21: F, t5589: F, t2719: F, t1041: F, t1251: F, t1691: F, t7093: F) -> (F, F, F, F, F, F, F) {
    let t7224 = t2722 * t626;
    let t7225 = t7224 * t422;
    let t7226 = t1815 * t7225;
    let t7228 = 8.0 / 45.0 * t639 * t7226;
    let t7230 = 4.0 / 15.0 * t561 * t5357;
    let t7231 = t213 * t626;
    let t7233 = t174 * t838 * t7231;
    let t7236 = t21 * t5589;
    let t7237 = t7236 * t2719;
    let t7239 = t1251 * t1041;
    let t7248 = t1691 * t7093;
    (t7228, t7230, t7233, t7236, t7237, t7239, t7248)
}
