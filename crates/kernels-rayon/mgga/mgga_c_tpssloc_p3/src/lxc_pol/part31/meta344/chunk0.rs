//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1252/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1252(t14506: f64, t3032: f64, t3129: f64, t3038: f64, t225: f64, t4658: f64, t4553: f64, t4559: f64, t4555: f64, t3199: f64, t3185: f64, t1057: f64, t14205: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14507 = t14506 * t3032;
    let t14508 = t14507 * t3129;
    let t14511 = t14507 * t3038;
    let t14529 = t4658 * t225;
    let t14545 = t4553 * t225;
    let t14552 = t4559 * t225;
    let t14555 = t4555 * t225;
    let t14608 = t14506 * t3199;
    let t14618 = t14506 * t3185;
    let t14651 = t14205 * t1057;
    (t14508, t14511, t14529, t14545, t14552, t14555, t14608, t14618, t14651)
}
