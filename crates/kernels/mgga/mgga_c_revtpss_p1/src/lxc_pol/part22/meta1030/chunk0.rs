//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3613/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3613<F: Float>(t20361: F, t3399: F, t20365: F, t16926: F, t5087: F, t1134: F, t20337: F, t3407: F, t20370: F, t20356: F, t58145: F, t58147: F, t68470: F, t68473: F, t68476: F, t68479: F, t68481: F, t68484: F) -> (F, F, F, F, F, F, F) {
    let t68486 = t20361 * t3399;
    let t68488 = t20365 * t3399;
    let t68490 = t5087 * t16926;
    let t68493 = t3407 * t20337 * t1134;
    let t68495 = t20370 * t3399;
    let t68497 = t20356 * t3399;
    let t68501 = -F::cast_from(0.485484375e1_f64) * t68470 + F::cast_from(0.19419375e1_f64) * t68473 + F::cast_from(0.6189328125e-1_f64) * t68476 - F::cast_from(0.412621875e-1_f64) * t68479 - F::cast_from(0.258925e1_f64) * t68481 - F::cast_from(0.258925e1_f64) * t68484 - F::cast_from(0.1294625e1_f64) * t68486 - F::cast_from(0.412621875e-1_f64) * t68488 + F::cast_from(0.16504875e0_f64) * t68490 + F::cast_from(0.16504875e0_f64) * t68493 + F::cast_from(0.82524375e-1_f64) * t68495 + F::cast_from(0.19419375e1_f64) * t68497 + F::cast_from(0.36793333333333333334e0_f64) * t58145 - F::cast_from(0.11038e0_f64) * t58147;
    (t68486, t68488, t68490, t68493, t68495, t68497, t68501)
}
