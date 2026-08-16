//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2478/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2478(t48486: f64, t13985: f64, t46740: f64, t13878: f64, t9765: f64, t14055: f64, t9775: f64, t1885: f64, t46722: f64, t14047: f64, t14051: f64, t1412: f64, t5658: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t48487 = 0.34013387707001991332e-1_f64 * t48486;
    let t48488 = t46740 * t13985;
    let t48489 = 0.16262400898971305032e-2_f64 * t48488;
    let t48508 = t9765 * t13878;
    let t48509 = 0.8131200449485652516e-2_f64 * t48508;
    let t48516 = t9775 * t14055;
    let t48518 = t46722 * t1885;
    let t48529 = t9775 * t14047;
    let t48531 = t9775 * t14051;
    let t48532 = 0.22866142996303859718e-3_f64 * t48531;
    let t48533 = t1412 * t5658;
    (t48487, t48489, t48509, t48516, t48518, t48529, t48532, t48533)
}
