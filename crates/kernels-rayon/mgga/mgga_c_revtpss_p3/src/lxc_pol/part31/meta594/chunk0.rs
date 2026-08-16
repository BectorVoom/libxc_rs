//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2021/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2021(t25953: f64, t26072: f64, t2435: f64, t25913: f64, t7289: f64, t94600: f64, t2028: f64, t3999: f64, t25875: f64, t25894: f64, t25877: f64, t94382: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t94756 = t26072 * t25953;
    let t94758 = t2435 * t25913;
    let t94761 = 0.39982213492741449076e-1_f64 * t7289 * t94600;
    let t94762 = t2028 * t3999;
    let t94763 = t25875 * t94762;
    let t94768 = t25894 * t94762;
    let t94771 = t94382 * t25877;
    (t94756, t94758, t94761, t94763, t94768, t94771)
}
