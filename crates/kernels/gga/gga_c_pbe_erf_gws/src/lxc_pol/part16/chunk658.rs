//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 658/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk658<F: Float>(t3189: F, t858: F, t2210: F, t884: F, t2494: F, t886: F, t2204: F, t3170: F, t3174: F, t3175: F, t3176: F, t3177: F, t3182: F, t3186: F, t3188: F) -> (F, F, F, F, F, F) {
    let t3190 = t858 * t3189;
    let t3191 = t2210 * t3190;
    let t3193 = t884 * t3191 / F::cast_from(16.0_f64);
    let t3194 = t858 * t2494;
    let t3195 = t886 * t3194;
    let t3197 = t884 * t3195 / F::cast_from(48.0_f64);
    let t3198 = -t3170 - t3174 - t3175 + t3176 + t2204 + t3177 - t3182 + t3186 + t3188 + t3193 - t3197;
    (t3190, t3191, t3193, t3195, t3197, t3198)
}
