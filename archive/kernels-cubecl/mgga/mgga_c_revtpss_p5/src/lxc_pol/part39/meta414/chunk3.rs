//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1495/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1495<F: Float>(t31027: F, t31268: F, t100: F, t101460: F, t101463: F, t10199: F, t116942: F, t117482: F, t117484: F, t117497: F, t117499: F, t117500: F, t117505: F, t1504: F, t2174: F, t2256: F, t2366: F, t31035: F, t31043: F, t31058: F, t31283: F, t4269: F, t8258: F, t8259: F, t8267: F, t8268: F) -> F {
    let t117510 = F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t31027 * t31268;
    let t117517 = -F::cast_from(5.0_f64) / F::cast_from(24.0_f64) * t10199 * t2174 * t100 - t117482 + t117484 + F::cast_from(5.0_f64) / F::cast_from(12.0_f64) * t8258 * t8268 * t1504 * t2366 + F::cast_from(25.0_f64) / F::cast_from(54.0_f64) * t8267 * t116942 * t31283 - F::cast_from(5.0_f64) / F::cast_from(36.0_f64) * t8267 * t31058 * t1504 * t2256 + t117497 - F::cast_from(5.0_f64) / F::cast_from(2.0_f64) * t117499 * t117500 * t31043 + F::cast_from(5.0_f64) / F::cast_from(9.0_f64) * t117505 * t4269 * t31043 - t117510 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t31035 * t8259 * t101460 - F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t31035 * t8259 * t101463;
    t117517
}
