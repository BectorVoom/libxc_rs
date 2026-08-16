//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 895/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk895(t1166: f64, t1979: f64, t1982: f64, t2313: f64, t7501: f64, t8562: f64, t2139: f64, t27: f64, t4928: f64, t649: f64, t35106: f64, t35110: f64, t35114: f64, t35118: f64, t39445: f64, t39449: f64, t39452: f64, t39453: f64, t39455: f64, t39457: f64, t39461: f64, t39463: f64, t39465: f64, t39470: f64, t8817: f64, t931: f64) -> f64 {
    let t39474 = t2313 * t1166 * t1979 * t1982;
    let t39482 = t7501 * t8562;
    let t39486 = t2139 * t27 * t649 * t4928;
    let t39488 = -0.31923449919973379548e-4_f64 * t39445 - 0.1064114997332445985e-4_f64 * t39449 + t39452 + 0.25538759935978703638e-4_f64 * t39453 - 0.53205749866622299248e-5_f64 * t39455 - 0.1064114997332445985e-4_f64 * t39457 - 0.25538759935978703638e-4_f64 * t39461 + 0.25538759935978703638e-4_f64 * t39463 + 0.31923449919973379548e-4_f64 * t39465 - 0.1064114997332445985e-4_f64 * t39470 + 0.42564599893297839398e-5_f64 * t39474 - 0.2363e1_f64 * t931 * t8817 - 0.15243824895787514157e-3_f64 * t35106 + 0.21684485328539747656e-4_f64 * t35110 - 0.30487649791575028314e-3_f64 * t35114 + 0.43368970657079495312e-4_f64 * t35118 - 0.27274661654245341728e-1_f64 * t39482 - 0.13637330827122670864e-1_f64 * t39486;
    t39488
}
