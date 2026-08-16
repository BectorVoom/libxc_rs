//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 308/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk308(t1455: f64, t504: f64, t503: f64, t475: f64, t1216: f64, t196: f64, t140: f64, t299: f64, t480: f64, t139: f64, t201: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1456 = t1455 * t504;
    let t1457 = t503 * t503;
    let t1458 = 1.0_f64 / t1457;
    let t1459 = t475 * t1458;
    let t1460 = t1216 * t196;
    let t1469 = 0.26531111111111111111e-1_f64 * t140 * t299 * t480;
    let t1470 = t139 * t201;
    (t1456, t1457, t1458, t1459, t1460, t1469, t1470)
}
