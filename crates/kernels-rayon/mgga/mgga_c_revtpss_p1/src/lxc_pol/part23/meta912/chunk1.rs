//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2933/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2933(t63533: f64, t63538: f64, t63541: f64, t63543: f64, t63545: f64, t63547: f64, t63549: f64, t63551: f64, t77829: f64, t77832: f64, t77835: f64, t77838: f64) -> f64 {
    let t77961 = -0.11577222222222222223e0_f64 * t63533 + 0.69463333333333333335e0_f64 * t63538 - 0.125034e1_f64 * t77829 + 0.62517e0_f64 * t77832 - 0.104195e0_f64 * t77835 - 0.104195e0_f64 * t77838 - 0.41678e0_f64 * t63541 + 0.69463333333333333333e-1_f64 * t63543 - 0.34731666666666666667e0_f64 * t63545 - 0.41678000000000000001e0_f64 * t63547 + 0.13892666666666666667e0_f64 * t63549 + 0.9261777777777777778e-1_f64 * t63551;
    t77961
}
