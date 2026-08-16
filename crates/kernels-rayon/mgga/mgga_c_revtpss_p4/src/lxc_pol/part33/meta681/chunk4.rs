//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2225/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2225(t5883: f64, t7583: f64, t108129: f64, t108681: f64, t108685: f64, t108687: f64, t108691: f64, t108693: f64, t108712: f64, t108716: f64, t108718: f64, t108721: f64, t108723: f64, t108725: f64, t108727: f64, t1310: f64, t2163: f64, t21814: f64, t21882: f64, t30724: f64, t508: f64, t5517: f64, t5877: f64, t7586: f64, t7683: f64, t8152: f64) -> (f64, f64) {
    let t111708 = t7583 * t5883;
    let t111717 = -2.0_f64 * t111708 * t508 - 2.0_f64 * t1310 * t30724 - t2163 * t21814 - 2.0_f64 * t21882 * t7586 - 2.0_f64 * t5517 * t8152 - t5877 * t7683 - t108129 + t108681 - t108685 + t108687 + t108691 + t108693 - t108712 - t108716 - t108718 - t108721 - t108723 - t108725 - t108727;
    (t111708, t111717)
}
