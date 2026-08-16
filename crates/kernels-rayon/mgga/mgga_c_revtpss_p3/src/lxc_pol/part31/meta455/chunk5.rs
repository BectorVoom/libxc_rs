//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1642/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1642(t5378: f64, t5391: f64, t17459: f64, t6688: f64, t3720: f64, t5405: f64, t6421: f64, t12787: f64, t17394: f64, t4890: f64, t3767: f64, t3782: f64) -> (f64, f64, f64, f64, f64) {
    let t21001 = t5391 * t5378;
    let t21003 = t6688 * t17459;
    let t21004 = t3720 * t21003;
    let t21007 = t6421 * t5405;
    let t21008 = t12787 * t21007;
    let t21013 = t17394 * t4890;
    let t21014 = t3767 * t21013;
    let t21017 = t3782 * t21013;
    (t21001, t21004, t21008, t21014, t21017)
}
