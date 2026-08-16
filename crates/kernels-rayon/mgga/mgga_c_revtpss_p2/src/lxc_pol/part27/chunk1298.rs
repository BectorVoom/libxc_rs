//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1298/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1298(t2247: f64, t26781: f64, t38: f64, t2123: f64, t25102: f64, t25110: f64, t25114: f64, t26749: f64, t26755: f64, t6960: f64, t7566: f64, t7576: f64, t7579: f64, t92654: f64, t92658: f64, t92709: f64, t92711: f64) -> f64 {
    let t96792 = t2247 * t38 * t26781;
    let t96803 = 5.0_f64 * t26749 * t25110 + 5.0_f64 / 2.0_f64 * t26749 * t25114 + t92709 * t2123 + t92711 * t2123 + 2.0_f64 * t25102 * t7576 + 2.0_f64 * t25102 * t7579 + 5.0_f64 / 2.0_f64 * t96792 * t6960 + 5.0_f64 * t26755 * t25110 + 5.0_f64 / 2.0_f64 * t26755 * t25114 + 5.0_f64 / 2.0_f64 * t7566 * t92654 + 5.0_f64 / 2.0_f64 * t7566 * t92658;
    t96803
}
