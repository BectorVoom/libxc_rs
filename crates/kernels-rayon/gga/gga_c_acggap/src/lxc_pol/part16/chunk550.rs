//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 550/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk550(t3088: f64, t4184: f64, t1642: f64, t3378: f64, t1539: f64, t4166: f64, t1160: f64, t1630: f64, t3077: f64, t150: f64, t2934: f64, t119: f64) -> (f64, f64, f64, f64, f64) {
    let t4185 = t3088 * t4184;
    let t4188 = 0.13170898365871023197e1_f64 * t3378 * t1642;
    let t4189 = t4166 * t1539;
    let t4191 = 0.13170898365871023197e1_f64 * t1160 * t4189;
    let t4192 = t3077 * t1630;
    let t4197 = t150 * t2934;
    let t4198 = t119 * t4197;
    (t4185, t4188, t4191, t4192, t4198)
}
