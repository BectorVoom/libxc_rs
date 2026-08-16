//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1378/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1378(t127: f64, t371: f64, t5318: f64, t1235: f64, t1803: f64, t3670: f64, t3685: f64, t5373: f64, t140: f64, t5368: f64, t1222: f64, t3624: f64, t5436: f64) -> (f64, f64, f64, f64, f64) {
    let t17435 = t371 * t127 * t5318;
    let t17437 = 0.28582678745379824648e-3_f64 * t1235 * t17435;
    let t17438 = t3670 * t1803;
    let t17444 = t5373 * t3685 / 162.0_f64;
    let t17445 = t140 * t5368;
    let t17447 = t1222 * t17445 / 432.0_f64;
    let t17448 = t5436 * t3624;
    (t17437, t17438, t17444, t17447, t17448)
}
