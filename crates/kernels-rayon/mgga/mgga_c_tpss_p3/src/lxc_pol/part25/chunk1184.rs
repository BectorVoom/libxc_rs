//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1184/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1184(t17944: f64, t17971: f64, t219: f64, t5832: f64, t1805: f64, t768: f64) -> (f64, f64, f64, f64) {
    let t18737 = 35.0_f64 / 216.0_f64 * t17944;
    let t18746 = 119.0_f64 / 3456.0_f64 * t17971;
    let t18753 = t5832 * t219;
    let t18770 = t768 * t1805;
    (t18737, t18746, t18753, t18770)
}
