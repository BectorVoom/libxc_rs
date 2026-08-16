//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1231/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1231(t108: f64, t5911: f64, t105: f64, t109: f64, t1507: f64, t1510: f64, t5896: f64, t5899: f64, t5902: f64, t5908: f64, t97: f64) -> (f64, f64) {
    let t5912 = t108 * t5911;
    let t5915 = 10.0_f64 / 9.0_f64 * t97 * t5896 + 5.0_f64 / 3.0_f64 * t97 * t5899 + 40.0_f64 / 9.0_f64 * t5902 * t109 - 50.0_f64 / 9.0_f64 * t1507 * t1510 + 10.0_f64 / 9.0_f64 * t105 * t5908 + 5.0_f64 / 3.0_f64 * t105 * t5912;
    (t5912, t5915)
}
