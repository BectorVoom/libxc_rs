//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1045/1444 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1045(t12058: f64, t1616: f64, t11314: f64, t11318: f64, t11323: f64, t11327: f64, t11334: f64, t11337: f64, t11339: f64, t11345: f64, t11348: f64, t11351: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12059 = t1616 * t12058;
    let t12060 = 2.0_f64 * t12059;
    let t12068 = 0.34752370105806885418e-3_f64 * t11314;
    let t12069 = 0.34752370105806885418e-3_f64 * t11318;
    let t12070 = 0.51491428373437201895e-5_f64 * t11323;
    let t12071 = 0.70344136651018351213e-8_f64 * t11327;
    let t12073 = 0.25340269868817520617e-3_f64 * t11334;
    let t12074 = 0.25301920572916666668e-5_f64 * t11337;
    let t12075 = 0.40483072916666666669e-4_f64 * t11339;
    let t12076 = 0.24458523220486111112e-4_f64 * t11345;
    let t12077 = 0.34752370105806885418e-3_f64 * t11348;
    let t12078 = 0.40483072916666666669e-4_f64 * t11351;
    (t12059, t12060, t12068, t12069, t12070, t12071, t12073, t12074, t12075, t12076, t12077, t12078)
}
