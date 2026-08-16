//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1891/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1891(t27940: f64, t5677: f64, t26028: f64, t5697: f64, t5701: f64, t5706: f64, t5614: f64, t7271: f64, t5661: f64, t7264: f64, t25997: f64, t5665: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t27941 = t27940 * t5677;
    let t27943 = t26028 * t5697;
    let t27945 = t26028 * t5701;
    let t27947 = t26028 * t5706;
    let t27949 = t7271 * t5614;
    let t27951 = t7264 * t5661;
    let t27953 = t25997 * t5665;
    (t27941, t27943, t27945, t27947, t27949, t27951, t27953)
}
