//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3032/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3032(t10778: f64, t221: f64, t10777: f64, t14792: f64, t2659: f64, t4503: f64, t816: f64, t14803: f64, t50769: f64, t14931: f64, t4372: f64, t9784: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t51123 = t10778 * t221;
    let t51125 = t10777 * t51123 * t14792;
    let t51133 = t816 * t2659 * t4503;
    let t51135 = t51133 * t50769 * t14803;
    let t51168 = t14931 * t51123 * t14803;
    let t51170 = t9784 * t4372;
    (t51123, t51125, t51133, t51135, t51168, t51170)
}
