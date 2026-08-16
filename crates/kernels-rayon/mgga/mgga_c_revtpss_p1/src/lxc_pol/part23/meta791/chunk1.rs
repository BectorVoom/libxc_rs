//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2608/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2608(t18413: f64, t2661: f64, t2662: f64, t837: f64, t10716: f64, t18402: f64, t10722: f64, t5993: f64, t18481: f64, t50768: f64, t51176: f64, t18333: f64, t50769: f64) -> (f64, f64, f64, f64, f64) {
    let t61673 = t2661 * t2662 * t18413 * t837;
    let t61675 = t10716 * t18402;
    let t61677 = t10722 * t5993;
    let t61689 = t50768 * t51176 * t18481;
    let t61692 = t50768 * t50769 * t18333;
    (t61673, t61675, t61677, t61689, t61692)
}
