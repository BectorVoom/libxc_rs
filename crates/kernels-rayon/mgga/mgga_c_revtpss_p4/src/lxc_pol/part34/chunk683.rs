//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 683/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk683(t1868: f64, t5532: f64, t3854: f64, t3859: f64, t3862: f64, t3865: f64, t3867: f64, t3871: f64, t3873: f64, t4027: f64, t4035: f64, t4037: f64, t4042: f64, t4139: f64, t6827: f64, t6828: f64) -> f64 {
    let t6930 = t5532 * t1868;
    let t6933 = 6.0_f64 * t4139 * t6930 + t3854 + t3859 + t3862 + t3865 - t3867 + t3871 + t3873 - t4027 - t4035 - t4037 + t4042 + t6827 - t6828;
    t6933
}
