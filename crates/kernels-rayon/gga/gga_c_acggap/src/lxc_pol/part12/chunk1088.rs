//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1088/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1088(t2314: f64, t31258: f64, t1982: f64, t568: f64, t142: f64, t4487: f64, t13299: f64, t31057: f64, t35288: f64, t4643: f64, t7486: f64, t2095: f64) -> (f64, f64, f64, f64, f64) {
    let t35359 = t31258 * t2314;
    let t35364 = t568 * t1982;
    let t35366 = t35364 * t142 * t4487;
    let t35379 = t31057 * t13299 * t35288;
    let t35383 = t4643 * t7486;
    let t35384 = t2095 * t35383;
    (t35359, t35366, t35379, t35383, t35384)
}
