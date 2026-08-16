//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3244/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3244(t14923: f64, t18634: f64, t10726: f64, t18408: f64, t2661: f64, t4366: f64, t18608: f64, t2662: f64, t837: f64, t18632: f64, t4352: f64, t10815: f64, t6019: f64) -> (f64, f64, f64, f64, f64) {
    let t61550 = t14923 * t18634;
    let t61560 = t2661 * t10726 * t18408 * t4366;
    let t61564 = t2661 * t2662 * t18608 * t837;
    let t61568 = t2661 * t10726 * t4352 * t18632;
    let t61570 = t10815 * t6019;
    (t61550, t61560, t61564, t61568, t61570)
}
