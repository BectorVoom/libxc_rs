//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 549/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk549(t1386: f64, t5737: f64, t1364: f64, t3729: f64, t3731: f64, t4143: f64, t507: f64, t5418: f64, t5623: f64, t5630: f64, t5635: f64, t5639: f64, t5646: f64, t5651: f64, t5657: f64, t5665: f64, t5669: f64, t5674: f64, t5679: f64, t5681: f64, t5684: f64, t5686: f64) -> (f64, f64) {
    let t5738 = t5737 * t1386;
    let t5741 = -0.24872916666666666666e-2_f64 * t5418 - 0.24872916666666666666e-2_f64 * t5630 + 0.66327777777777777776e-2_f64 * t5635 + 0.11054629629629629629e-2_f64 * t5639 - 0.16581944444444444444e-2_f64 * t3729 + 0.11054629629629629629e-2_f64 * t3731 - 0.16581944444444444444e-2_f64 * t5646 + 0.11054629629629629629e-2_f64 * t5651 - 0.33163888888888888888e-2_f64 * t5657 + 0.27636574074074074073e-2_f64 * t5665 - 0.16581944444444444444e-2_f64 * t5669 - 0.16581944444444444444e-2_f64 * t5674 + 0.49745833333333333332e-2_f64 * t5679 + 0.11054629629629629629e-2_f64 * t5681 + 0.11054629629629629629e-2_f64 * t4143 - 0.44218518518518518517e-2_f64 * t5684 + 0.16581944444444444444e-2_f64 * t5686 + t5623 * t507 - 0.66725e-1_f64 * t1364 * t5738;
    (t5738, t5741)
}
