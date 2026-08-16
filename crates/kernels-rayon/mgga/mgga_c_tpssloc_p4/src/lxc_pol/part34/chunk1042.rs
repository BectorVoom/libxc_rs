//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1042/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1042(t22761: f64, t6390: f64, t2002: f64, t6378: f64, t559: f64, t6422: f64, t6945: f64, t6427: f64, t6952: f64, t6431: f64, t1831: f64, t26257: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t28085 = t22761 * t6390;
    let t28088 = t6378 * t2002;
    let t28089 = t28088 * t559;
    let t28091 = t6945 * t6422;
    let t28093 = t6952 * t6427;
    let t28095 = t6952 * t6431;
    let t28097 = t26257 * t1831;
    (t28085, t28088, t28089, t28091, t28093, t28095, t28097)
}
