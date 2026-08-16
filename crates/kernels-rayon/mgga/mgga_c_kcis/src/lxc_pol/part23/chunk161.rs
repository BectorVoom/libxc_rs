//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 161/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk161(t661: f64, t662: f64, t646: f64, t2: f64, t45: f64, t56: f64, t649: f64, t88: f64, t47: f64, t52: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t663 = t661 * t662;
    let t665 = 1.0_f64 * t646 * t663;
    let t666 = t45 * t2;
    let t668 = t649 * t88 * t56;
    let t671 = t45 * t47;
    let t672 = t52 * t52;
    let t673 = 1.0_f64 / t672;
    (t663, t665, t666, t668, t671, t672, t673)
}
