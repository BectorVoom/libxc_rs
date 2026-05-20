//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3169/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3169<F: Float>(t1803: F, t20923: F, t21334: F, t44291: F, t484: F, t5261: F, t6594: F, t70225: F, t70250: F, t70263: F, t70265: F, t70270: F, t70273: F, t70275: F, t70647: F) -> F {
    let t83259 = F::cast_from(0.21722835846488666732e-1_f64) * t5261 * t6594 * t484 - F::cast_from(0.34299214494455789577e-2_f64) * t21334 * t1803 * t484 + F::cast_from(0.63517063878621832551e-4_f64) * t44291 - t70225 / F::new(324.0) + F::cast_from(0.7622047665434619906e-2_f64) * t70647 * t20923 + F::cast_from(0.25724410870841842183e-2_f64) * t70250 + F::cast_from(0.14291339372689912324e-3_f64) * t70263 + F::cast_from(0.45732285992607719436e-2_f64) * t70265 - F::cast_from(0.11433071498151929859e-2_f64) * t70270 + F::cast_from(0.47637797908966374413e-3_f64) * t70273 - F::cast_from(0.42874018118069736972e-3_f64) * t70275;
    t83259
}
