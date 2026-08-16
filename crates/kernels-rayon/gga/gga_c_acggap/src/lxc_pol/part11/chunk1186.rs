//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1186/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1186(t30248: f64, t537: f64, t7637: f64, t8859: f64, t1576: f64, t7614: f64, t13299: f64, t33952: f64, t33954: f64, t15386: f64, t31443: f64, t35704: f64) -> (f64, f64, f64, f64, f64) {
    let t36236 = t30248 * t537;
    let t36238 = t7637 * t8859;
    let t36240 = t7614 * t1576;
    let t36243 = t33952 * t13299 * t33954;
    let t36246 = t31443 * t15386 * t35704;
    (t36236, t36238, t36240, t36243, t36246)
}
