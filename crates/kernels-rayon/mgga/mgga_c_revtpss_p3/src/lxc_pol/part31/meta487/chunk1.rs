//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1781/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1781(t3244: f64, t7111: f64, t3111: f64, t7132: f64, t1058: f64, t7126: f64, t1973: f64, t3201: f64, t7114: f64, t1020: f64, t7131: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25543 = t7111 * t3244;
    let t25551 = t7132 * t3111;
    let t25557 = t7126 * t1058;
    let t25560 = 0.95275595817932748827e-4_f64 * t1973 * t3201;
    let t25564 = t7114 * t1058;
    let t25569 = t1020 * t7131;
    (t25543, t25551, t25557, t25560, t25564, t25569)
}
