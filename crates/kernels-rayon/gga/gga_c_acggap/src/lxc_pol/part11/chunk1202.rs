//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1202/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1202(t2138: f64, t2147: f64, t322: f64, t8392: f64, t7998: f64, t8397: f64, t1658: f64, t2122: f64, t2146: f64, t31965: f64, t32124: f64, t32133: f64, t32135: f64, t32143: f64, t32150: f64, t32157: f64, t33535: f64, t36432: f64, t36436: f64, t36439: f64, t36447: f64, t7934: f64, t9026: f64) -> f64 {
    let t36452 = 0.34694512752820797848e1_f64 * t2138 * t2147 * t8392 * t322;
    let t36460 = t8397 * t7998;
    let t36463 = -t36432 - t36436 + t36439 - 0.34694512752820797848e1_f64 * t32133 + 0.52041769129231196772e1_f64 * t32124 * t33535 * t7934 - t36447 + 0.13170898365871023197e1_f64 * t32135 - t36452 + 0.69389025505641595696e1_f64 * t32143 - 0.17347256376410398924e1_f64 * t31965 * t9026 + t32150 + 0.17347256376410398924e1_f64 * t2146 * t2147 * t2122 * t1658 - 0.8673628188205199462e0_f64 * t36460 + 0.17347256376410398924e1_f64 * t32157;
    t36463
}
