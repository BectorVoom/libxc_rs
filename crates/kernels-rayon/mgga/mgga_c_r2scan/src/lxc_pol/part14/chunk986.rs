//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 986/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk986(t3332: f64, t7629: f64, t7628: f64, t8156: f64, t6165: f64, t8160: f64, t7615: f64, t7614: f64, t3610: f64, t6395: f64, t8066: f64, t2147: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11640 = t3332 * t7629;
    let t11641 = t7628 * t11640;
    let t11643 = t3332 * t8156;
    let t11644 = t6165 * t11643;
    let t11646 = t3332 * t8160;
    let t11647 = t6165 * t11646;
    let t11649 = t3332 * t7615;
    let t11650 = t7614 * t11649;
    let t11652 = t6395 * t3610;
    let t11654 = t3332 * t8066;
    let t11655 = t2147 * t11654;
    (t11640, t11641, t11643, t11644, t11646, t11647, t11649, t11650, t11652, t11654, t11655)
}
