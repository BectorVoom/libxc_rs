//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1567/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1567(t3453: f64, t5146: f64, t3479: f64, t5142: f64, t1168: f64, t3471: f64, t12472: f64, t1744: f64, t1757: f64, t3497: f64, t1745: f64, t1187: f64, t5181: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16955 = t5146 * t3453;
    let t16958 = t5142 * t3479;
    let t16959 = t16958 * t1168;
    let t16962 = t5146 * t3471;
    let t16965 = t1744 * t12472;
    let t16966 = t16965 * t3453;
    let t16971 = t1757 * t3497;
    let t16974 = t1745 * t3453;
    let t16979 = t5181 * t1187;
    (t16955, t16959, t16962, t16966, t16971, t16974, t16979)
}
