//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2394/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2394(t222: f64, t40735: f64, t10777: f64, t10779: f64, t2749: f64, t40578: f64, t10794: f64, t10811: f64, t10807: f64, t10709: f64, t10760: f64, t9794: f64) -> (f64, f64, f64, f64, f64) {
    let t40737 = 455.0_f64 / 243.0_f64 * t40735 * t222;
    let t40744 = t10777 * t10779 * t40578 * t2749;
    let t40748 = t10811 * t10794;
    let t40750 = t10811 * t10807;
    let t40753 = t10760 * t9794 * t10709;
    (t40737, t40744, t40748, t40750, t40753)
}
