//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 659/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk659(t1399: f64, t221: f64, t4019: f64, t4018: f64, t1317: f64, t1331: f64, t1333: f64, t2522: f64, t2562: f64, t2569: f64, t2579: f64, t2587: f64, t3852: f64, t3854: f64, t3871: f64, t3873: f64) -> (f64, f64, f64, f64, f64) {
    let t4021 = t4019 * t221 * t1399;
    let t4022 = t4018 * t4021;
    let t4024 = t1317 * t1331;
    let t4025 = 8.0_f64 * t4024;
    let t4027 = 8.0_f64 * t1317 * t1333;
    let t4028 = t3873 - t2522 + t4025 + t4027 + t2579 + t2587 + t3871 + t3852 - t2562 - t2569 + t3854;
    (t4021, t4022, t4025, t4027, t4028)
}
