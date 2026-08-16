//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1276/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1276(t19577: f64, t22574: f64, t36533: f64, t8449: f64, t8944: f64, t26164: f64, t120103: f64, t120104: f64, t120107: f64, t120108: f64, t120111: f64, t120114: f64, t120138: f64, t120166: f64, t120171: f64, t120173: f64, t120176: f64, t120177: f64, t120658: f64, t120659: f64, t1774: f64, t1976: f64, t26098: f64, t31029: f64, t5361: f64, t574: f64, t6862: f64, t7451: f64, t8447: f64) -> f64 {
    let t120663 = 6.0_f64 * t22574 * t36533 * t19577;
    let t120664 = t8449 * t8944;
    let t120665 = t120664 * t26164;
    let t120667 = t8447 * t5361 + t120103 - 6.0_f64 * t120104 + t120107 - 4.0_f64 * t120108 - t120111 - t120114 - t31029 * t1774 - 2.0_f64 * t26098 * t1976 - 2.0_f64 * t7451 * t6862 + (t120138 + t120166) * t574 + t120171 + 12.0_f64 * t120173 - t120176 + 2.0_f64 * t120177 + t120658 - 2.0_f64 * t120659 + t120663 + 4.0_f64 * t120665;
    t120667
}
