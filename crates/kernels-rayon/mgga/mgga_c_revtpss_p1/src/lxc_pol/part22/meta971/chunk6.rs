//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3250/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3250(t18437: f64, t2652: f64, t2661: f64, t2662: f64, t4352: f64, t4424: f64, t18413: f64, t837: f64, t10716: f64, t18402: f64, t10722: f64, t5993: f64) -> (f64, f64, f64, f64, f64) {
    let t61660 = t2652 * t18437;
    let t61669 = t2661 * t2662 * t4352 * t4424;
    let t61673 = t2661 * t2662 * t18413 * t837;
    let t61675 = t10716 * t18402;
    let t61677 = t10722 * t5993;
    (t61660, t61669, t61673, t61675, t61677)
}
