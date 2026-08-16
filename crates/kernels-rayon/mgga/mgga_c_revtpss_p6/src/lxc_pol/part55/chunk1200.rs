//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1200/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1200(t28658: f64, t7742: f64, t28063: f64, t7359: f64, t34018: f64, t7235: f64, t34302: f64, t95088: f64, t2014: f64, t28176: f64, t32629: f64, t198: f64, t205: f64, t8656: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t127547 = 2.0_f64 * t28658 * t7742;
    let t127549 = 2.0_f64 * t7359 * t28063;
    let t127550 = t7235 * t34018;
    let t127556 = 3.0_f64 * t95088 * t34302;
    let t127559 = 3.0_f64 * t2014 * t32629 * t28176;
    let t127566 = t198 * t205 * t8656;
    (t127547, t127549, t127550, t127556, t127559, t127566)
}
