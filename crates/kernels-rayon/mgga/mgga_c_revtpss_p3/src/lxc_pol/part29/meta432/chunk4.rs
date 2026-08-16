//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1603/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1603(t1263: f64, t5245: f64, t1122: f64, t1042: f64, t1234: f64, t5390: f64, t3704: f64, t5293: f64, t1121: f64, t1214: f64, t606: f64, t1250: f64) -> (f64, f64, f64, f64) {
    let t17500 = t1263 * t5245;
    let t17501 = t17500 * t1122;
    let t17502 = t1042 * t17501;
    let t17505 = t1234 * t5390;
    let t17509 = 0.15244095330869239812e-2_f64 * t5293 * t3704;
    let t17512 = t1214 * t1121;
    let t17513 = t17512 * t606;
    let t17514 = t1250 * t17513;
    (t17502, t17505, t17509, t17514)
}
