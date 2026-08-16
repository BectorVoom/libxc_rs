//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1414/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1414(t14506: f64, t3032: f64, t3129: f64, t3038: f64, t1020: f64, t10937: f64, t10962: f64, t10982: f64, t10985: f64, t10994: f64, t11003: f64, t14235: f64, t14491: f64, t14495: f64, t14503: f64, t1618: f64, t3043: f64, t3057: f64, t3064: f64, t3070: f64, t3114: f64, t3123: f64, t3134: f64, t4579: f64, t4641: f64, t4644: f64, t4652: f64) -> f64 {
    let t14507 = t14506 * t3032;
    let t14508 = t14507 * t3129;
    let t14511 = t14507 * t3038;
    let t14523 = 5.0_f64 / 6912.0_f64 * t3070 * t14235 + t1020 * t14491 / 3072.0_f64 + t14495 + t10982 / 864.0_f64 + t10985 / 648.0_f64 - t10994 / 432.0_f64 - t10937 * t4579 / 432.0_f64 + t14503 + t4641 * t3123 / 3072.0_f64 + t14508 * t3134 / 1536.0_f64 - t14511 * t3043 / 3072.0_f64 + t4644 * t3057 / 4608.0_f64 + 5.0_f64 / 13824.0_f64 * t4644 * t3064 + t10962 * t1618 / 3072.0_f64 + t3114 * t4652 / 1536.0_f64 + t11003 / 2304.0_f64;
    t14523
}
