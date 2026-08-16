//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 814/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk814(t598: f64, t8978: f64, t1967: f64, t2299: f64, t2294: f64, t2137: f64, t8396: f64, t615: f64) -> (f64, f64, f64, f64, f64) {
    let t8979 = t598 * t8978;
    let t8981 = t1967 * t2299;
    let t8983 = t1967 * t2294;
    let t8998 = t2137 * t8396;
    let t9003 = t615 * t8396;
    (t8979, t8981, t8983, t8998, t9003)
}
