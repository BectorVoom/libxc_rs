//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 599/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk599<F: Float>(t3131: F, t3139: F, t3140: F, t3138: F, t1136: F, t2164: F, t2170: F, t2171: F, t2168: F, t3110: F, t3115: F, t3118: F, t3122: F, t3125: F, t3127: F, t3130: F, t3136: F) -> (F, F, F, F, F, F, F) {
    let t3142 = t3139 * t3131 * t3140;
    let t3144 = t3138 * t3142 / F::new(48.0);
    let t3145 = t2164 * t1136;
    let t3146 = F::new(7.0) / F::new(288.0) * t3145;
    let t3148 = t2170 * t3131 * t2171;
    let t3150 = t2168 * t3148 / F::new(48.0);
    let t3151 = t3110 + t3115 - t3118 + t3122 - t3125 - t3127 - t3130 - t3136 + t3144 + t3146 + t3150;
    (t3142, t3144, t3145, t3146, t3148, t3150, t3151)
}
