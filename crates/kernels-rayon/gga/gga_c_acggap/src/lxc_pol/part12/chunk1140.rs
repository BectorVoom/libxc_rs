//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1140/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1140(t13299: f64, t33952: f64, t33954: f64, t15386: f64, t31443: f64, t35704: f64, t17912: f64, t33953: f64, t5207: f64, t142: f64, t5160: f64, t7436: f64) -> (f64, f64, f64, f64) {
    let t36243 = t33952 * t13299 * t33954;
    let t36246 = t31443 * t15386 * t35704;
    let t36250 = t31443 * t17912 * t33953 * t5207;
    let t36253 = t7436 * t142 * t5160;
    (t36243, t36246, t36250, t36253)
}
