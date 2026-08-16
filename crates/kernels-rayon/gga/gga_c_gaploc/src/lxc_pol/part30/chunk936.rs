//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 936/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk936(t3327: f64, t6305: f64, t4261: f64, t7893: f64, t9074: f64, t2312: f64, t3351: f64, t7974: f64, t894: f64, t1063: f64, t9097: f64, t9100: f64, t9108: f64, t9111: f64, t9113: f64, t9115: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10184 = 0.28455006635676149599e-1_f64 * t6305 * t3327;
    let t10185 = t4261 * t7893;
    let t10186 = t9074 * t10185;
    let t10187 = 0.23712505529730124666e-2_f64 * t10186;
    let t10194 = t2312 * t3351;
    let t10195 = 0.11856252764865062333e-2_f64 * t10194;
    let t10196 = t894 * t7974;
    let t10198 = 0.28455006635676149599e-1_f64 * t1063 * t10196;
    let t10205 = -21.0_f64 / 256.0_f64 * t9097 + 147.0_f64 / 8192.0_f64 * t9100 - 63.0_f64 / 524288.0_f64 * t9108 + 21.0_f64 / 524288.0_f64 * t9111 - 49.0_f64 / 8192.0_f64 * t9113 + 7.0_f64 / 256.0_f64 * t9115;
    (t10184, t10185, t10187, t10195, t10196, t10198, t10205)
}
