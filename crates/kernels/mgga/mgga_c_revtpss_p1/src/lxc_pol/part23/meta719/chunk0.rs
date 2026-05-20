//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2478/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2478<F: Float>(t48486: F, t13985: F, t46740: F, t13878: F, t9765: F, t14055: F, t9775: F, t1885: F, t46722: F, t14047: F, t14051: F, t1412: F, t5658: F) -> (F, F, F, F, F, F, F, F) {
    let t48487 = F::cast_from(0.34013387707001991332e-1_f64) * t48486;
    let t48488 = t46740 * t13985;
    let t48489 = F::cast_from(0.16262400898971305032e-2_f64) * t48488;
    let t48508 = t9765 * t13878;
    let t48509 = F::cast_from(0.8131200449485652516e-2_f64) * t48508;
    let t48516 = t9775 * t14055;
    let t48518 = t46722 * t1885;
    let t48529 = t9775 * t14047;
    let t48531 = t9775 * t14051;
    let t48532 = F::cast_from(0.22866142996303859718e-3_f64) * t48531;
    let t48533 = t1412 * t5658;
    (t48487, t48489, t48509, t48516, t48518, t48529, t48532, t48533)
}
