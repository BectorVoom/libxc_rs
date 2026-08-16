//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1478/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1478(t1244: f64, t3594: f64, t71691: f64, t17628: f64, t5373: f64, t3655: f64, t6595: f64, t1222: f64, t6658: f64, t697: f64, t6662: f64, t1209: f64, t1284: f64, t6695: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t71699 = t3594 * t1244 * t71691;
    let t71718 = t5373 * t17628;
    let t71744 = t6595 * t3655;
    let t71928 = t1222 * t697 * t6658;
    let t71931 = t1222 * t697 * t6662;
    let t72267 = t1209 * t1284 * t6695;
    (t71699, t71718, t71744, t71928, t71931, t72267)
}
