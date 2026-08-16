//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1302/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1302(t10638: f64, t231: f64, t268: f64, t2798: f64, t675: f64, t2645: f64, t837: f64, t2782: f64, t2797: f64, t10115: f64, t883: f64, t2482: f64, t2811: f64, t39588: f64, t686: f64, t72: f64) -> (f64, f64, f64, f64, f64) {
    let t39617 = t2798 * t268 * t675 * t10638 * t231;
    let t39620 = t837 * t2645;
    let t39622 = t2782 * t2797 * t39620;
    let t39624 = t10115 * t883;
    let t39629 = t2482 * t2811 * t72 * t686 * t39588;
    (t39617, t39620, t39622, t39624, t39629)
}
