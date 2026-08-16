//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1073/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1073(t1659: f64, t8331: f64, t33698: f64, t33699: f64, t638: f64, t315: f64, t323: f64, t9367: f64, t38092: f64, t7963: f64, t7965: f64, t4210: f64, t7942: f64) -> (f64, f64, f64, f64, f64) {
    let t38251 = 0.13170898365871023197e1_f64 * t8331 * t1659;
    let t38256 = 0.10408353825846239354e2_f64 * t33698 * t638 * t33699;
    let t38259 = 0.13170898365871023197e1_f64 * t315 * t9367 * t323;
    let t38280 = 0.17347256376410398924e1_f64 * t7963 * t38092 * t7965;
    let t38283 = 0.17347256376410398924e1_f64 * t7942 * t38092 * t4210;
    (t38251, t38256, t38259, t38280, t38283)
}
