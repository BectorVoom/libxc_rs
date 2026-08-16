//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1240/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1240(t7696: f64, t7953: f64, t7950: f64, t2170: f64, t28271: f64, t2042: f64, t29480: f64, t127489: f64, t127490: f64, t127492: f64, t127495: f64, t32373: f64, t34011: f64, t5805: f64, t8771: f64) -> f64 {
    let t129555 = t7696 * t7953;
    let t129559 = t7696 * t7950;
    let t129562 = t2170 * t28271;
    let t129564 = t29480 * t2042;
    let t129566 = 3.0_f64 * t5805 * t8771 + t127489 + 6.0_f64 * t127490 + 3.0_f64 * t127492 + t127495 + 3.0_f64 * t129555 + 6.0_f64 * t129559 + 6.0_f64 * t129562 + 3.0_f64 * t129564 + t32373 + t34011;
    t129566
}
