//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3169/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3169(t1803: f64, t20923: f64, t21334: f64, t44291: f64, t484: f64, t5261: f64, t6594: f64, t70225: f64, t70250: f64, t70263: f64, t70265: f64, t70270: f64, t70273: f64, t70275: f64, t70647: f64) -> f64 {
    let t83259 = 0.21722835846488666732e-1_f64 * t5261 * t6594 * t484 - 0.34299214494455789577e-2_f64 * t21334 * t1803 * t484 + 0.63517063878621832551e-4_f64 * t44291 - t70225 / 324.0_f64 + 0.7622047665434619906e-2_f64 * t70647 * t20923 + 0.25724410870841842183e-2_f64 * t70250 + 0.14291339372689912324e-3_f64 * t70263 + 0.45732285992607719436e-2_f64 * t70265 - 0.11433071498151929859e-2_f64 * t70270 + 0.47637797908966374413e-3_f64 * t70273 - 0.42874018118069736972e-3_f64 * t70275;
    t83259
}
