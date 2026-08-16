//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 764/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk764(t10138: f64, t335: f64, t333: f64, t3110: f64, t317: f64, t319: f64, t3072: f64, t311: f64, t10112: f64, t313: f64, t1031: f64, t1068: f64) -> (f64, f64, f64, f64, f64) {
    let t10139 = t10138 * t335;
    let t10141 = 0.72818958333333333333e-4_f64 * t333 * t10139;
    let t10144 = 0.27323333333333333333e-1_f64 * t317 * t3110 * t319;
    let t10170 = 1.0_f64 / t3072 / t311;
    let t10187 = 0.14055920378328537299e-1_f64 * t10112 * t313;
    let t10188 = t1068 * t1031;
    (t10141, t10144, t10170, t10187, t10188)
}
