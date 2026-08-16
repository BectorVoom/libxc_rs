//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 895/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk895<F: Float>(t1166: F, t1979: F, t1982: F, t2313: F, t7501: F, t8562: F, t2139: F, t27: F, t4928: F, t649: F, t35106: F, t35110: F, t35114: F, t35118: F, t39445: F, t39449: F, t39452: F, t39453: F, t39455: F, t39457: F, t39461: F, t39463: F, t39465: F, t39470: F, t8817: F, t931: F) -> F {
    let t39474 = t2313 * t1166 * t1979 * t1982;
    let t39482 = t7501 * t8562;
    let t39486 = t2139 * t27 * t649 * t4928;
    let t39488 = -F::cast_from(0.31923449919973379548e-4_f64) * t39445 - F::cast_from(0.1064114997332445985e-4_f64) * t39449 + t39452 + F::cast_from(0.25538759935978703638e-4_f64) * t39453 - F::cast_from(0.53205749866622299248e-5_f64) * t39455 - F::cast_from(0.1064114997332445985e-4_f64) * t39457 - F::cast_from(0.25538759935978703638e-4_f64) * t39461 + F::cast_from(0.25538759935978703638e-4_f64) * t39463 + F::cast_from(0.31923449919973379548e-4_f64) * t39465 - F::cast_from(0.1064114997332445985e-4_f64) * t39470 + F::cast_from(0.42564599893297839398e-5_f64) * t39474 - F::cast_from(0.2363e1_f64) * t931 * t8817 - F::cast_from(0.15243824895787514157e-3_f64) * t35106 + F::cast_from(0.21684485328539747656e-4_f64) * t35110 - F::cast_from(0.30487649791575028314e-3_f64) * t35114 + F::cast_from(0.43368970657079495312e-4_f64) * t35118 - F::cast_from(0.27274661654245341728e-1_f64) * t39482 - F::cast_from(0.13637330827122670864e-1_f64) * t39486;
    t39488
}
