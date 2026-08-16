//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 829/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk829(t5709: f64, t5909: f64, t5714: f64, t5724: f64, t5717: f64, t5722: f64, t5729: f64) -> (f64, f64, f64, f64) {
    let t5910 = t5909 * t5709;
    let t5913 = 7.0_f64 / 144.0_f64 * t5714;
    let t5916 = 7.0_f64 / 1152.0_f64 * t5724;
    let t5918 = -t5913 - t5717 / 24.0_f64 - t5722 / 768.0_f64 - t5916 - t5729 / 192.0_f64;
    (t5910, t5913, t5916, t5918)
}
