//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 785/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk785<F: Float>(t4610: F, t5250: F, t519: F, t4218: F, t4220: F, t4225: F, t4227: F, t4235: F, t5213: F, t5217: F, t5224: F, t5228: F, t5233: F, t5236: F, t5240: F, t5242: F, t5246: F, t5249: F) -> (F, F, F) {
    let t5251 = t5250 * t4610;
    let t5253 = F::cast_from(32.0_f64) / F::cast_from(81.0_f64) * t519 * t5251;
    let t5254 = F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t4218 + F::cast_from(16.0_f64) / F::cast_from(3.0_f64) * t4220 + t5213 + t5217 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t4225 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t4227 + t4235 - t5224 + t5228 + t5233 - t5236 + t5240 - t5242 - t5246 + t5249 + t5253;
    (t5251, t5253, t5254)
}
