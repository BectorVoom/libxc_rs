//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1200/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1200(t127489: f64, t127495: f64, t127503: f64, t127507: f64, t129555: f64, t129559: f64, t129562: f64, t129564: f64, t129570: f64, t129572: f64, t129574: f64, t129577: f64, t1461: f64, t34011: f64, t34014: f64, t35027: f64, t5802: f64, t5805: f64, t8616: f64, t8975: f64) -> f64 {
    let t132167 = 3.0_f64 * t1461 * t35027 + 6.0_f64 * t5802 * t8975 + 3.0_f64 * t5805 * t8975 + t127489 + t127495 + t127503 + t127507 + 6.0_f64 * t129555 + 12.0_f64 * t129559 + 12.0_f64 * t129562 + 6.0_f64 * t129564 + 12.0_f64 * t129570 + 12.0_f64 * t129572 + 12.0_f64 * t129574 + 6.0_f64 * t129577 + t34011 + t34014 + t8616;
    t132167
}
