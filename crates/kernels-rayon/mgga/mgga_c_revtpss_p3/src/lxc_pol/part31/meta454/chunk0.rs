//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1632/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1632(t1248: f64, t6573: f64, t1250: f64, t3720: f64, t19666: f64, t5302: f64, t1042: f64, t17550: f64, t19661: f64, t1715: f64, t17500: f64, t5056: f64, t5277: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20856 = t6573 * t1248;
    let t20857 = t20856 * t1250;
    let t20858 = t3720 * t20857;
    let t20863 = t5302 * t19666;
    let t20864 = t1042 * t20863;
    let t20867 = t17550 * t19661;
    let t20868 = t1042 * t20867;
    let t20875 = t17500 * t1715;
    let t20876 = t1042 * t20875;
    let t20879 = t5277 * t5056;
    (t20856, t20858, t20864, t20868, t20876, t20879)
}
