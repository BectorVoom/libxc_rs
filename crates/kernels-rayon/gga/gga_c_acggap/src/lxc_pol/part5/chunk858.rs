//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 858/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk858(t3889: f64, t868: f64, t441: f64, t848: f64, t464: f64, t3037: f64, t3922: f64, t449: f64, t463: f64, t1220: f64, t1221: f64, t863: f64, t864: f64) -> (f64, f64, f64, f64, f64) {
    let t12218 = t868 * t3889;
    let t12224 = t848 * t441;
    let t12225 = t12224 * t464;
    let t12229 = t3922 * t449 * t3037 * t463;
    let t12233 = t863 * t1220 * t864 * t1221;
    (t12218, t12224, t12225, t12229, t12233)
}
