//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3019/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3019(t14860: f64, t2661: f64, t2662: f64, t837: f64, t2646: f64, t4352: f64, t14652: f64, t4416: f64, t14663: f64, t221: f64, t2484: f64, t2485: f64) -> (f64, f64, f64, f64, f64) {
    let t50732 = t2661 * t2662 * t14860 * t837;
    let t50736 = t2661 * t2662 * t4352 * t2646;
    let t50740 = t2661 * t2662 * t14652 * t837;
    let t50744 = t2661 * t2662 * t4416 * t2646;
    let t50748 = t2484 * t2485 * t221 * t14663;
    (t50732, t50736, t50740, t50744, t50748)
}
