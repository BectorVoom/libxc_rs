//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 349/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk349(t1737: f64, t1744: f64, t1746: f64, t1701: f64, t1706: f64, t1726: f64, t1731: f64, t1735: f64, t45: f64, t621: f64, t634: f64, t67: f64) -> (f64, f64) {
    let t1747 = t1737 * t1744 * t1746;
    let t1750 = -0.62182e-1_f64 * t1701 * t621 + 1.0_f64 * t1706 * t1726 + 0.19751789702565206229e-1_f64 * t45 * t1731 * t634 - 0.58482233974552040708e0_f64 * t1735 * t1747;
    let t1751 = t67 * t1750;
    (t1747, t1751)
}
