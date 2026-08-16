//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1230/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1230(t545: f64, t94667: f64, t25875: f64, t25894: f64, t26069: f64, t94407: f64, t1426: f64, t9990: f64, t7282: f64, t9646: f64, t2022: f64, t22: f64, t25937: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t94668 = t94667 * t545;
    let t94669 = t25875 * t94668;
    let t94674 = t25894 * t94668;
    let t94682 = 0.91399340044406952588e-2_f64 * t26069 * t94407;
    let t94683 = t1426 * t9990;
    let t94696 = t9646 * t7282;
    let t94698 = t25937 * t2022 * t22;
    (t94669, t94674, t94682, t94683, t94696, t94698)
}
