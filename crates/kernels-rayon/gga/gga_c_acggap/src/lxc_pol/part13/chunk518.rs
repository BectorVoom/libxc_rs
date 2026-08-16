//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 518/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk518(t1237: f64, t3077: f64, t1236: f64, t955: f64, t1160: f64, t929: f64, t944: f64, t1159: f64, t862: f64) -> (f64, f64, f64, f64) {
    let t3078 = t3077 * t1237;
    let t3080 = t1236 * t955;
    let t3081 = t1160 * t3080;
    let t3084 = t944 * t929;
    let t3088 = t862 * t1159;
    (t3078, t3081, t3084, t3088)
}
