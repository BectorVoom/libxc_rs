//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 969/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk969(t11643: f64, t6165: f64, t3332: f64, t8160: f64, t7615: f64, t7614: f64, t3610: f64, t6395: f64, t8066: f64, t2147: f64, t3316: f64, t980: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11644 = t6165 * t11643;
    let t11646 = t3332 * t8160;
    let t11647 = t6165 * t11646;
    let t11649 = t3332 * t7615;
    let t11650 = t7614 * t11649;
    let t11652 = t6395 * t3610;
    let t11654 = t3332 * t8066;
    let t11655 = t2147 * t11654;
    let t11657 = t980 * t3316;
    (t11644, t11646, t11647, t11649, t11650, t11652, t11654, t11655, t11657)
}
