//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2758/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2758(t808: f64, t853: f64, t14792: f64, t50768: f64, t14688: f64, t40731: f64, t10777: f64, t14671: f64, t14686: f64, t2754: f64, t14749: f64, t221: f64) -> (f64, f64, f64, f64, f64) {
    let t50769 = t808 * t853;
    let t50771 = t50768 * t50769 * t14792;
    let t50773 = t40731 * t14688;
    let t50774 = 0.40656002247428262579e-3_f64 * t50773;
    let t50784 = t10777 * t14686 * t14671 * t2754;
    let t50789 = t221 * t14749;
    (t50769, t50771, t50774, t50784, t50789)
}
