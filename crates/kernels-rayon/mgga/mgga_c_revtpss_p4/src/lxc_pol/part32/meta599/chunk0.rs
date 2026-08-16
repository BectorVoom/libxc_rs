//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1934/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1934(t5966: f64, t605: f64, t27375: f64, t98658: f64, t1468: f64, t4343: f64, t5962: f64, t6075: f64, t775: f64, t25207: f64, t1583: f64, t580: f64, t98631: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t105902 = t605 * t5966;
    let t105906 = t98658 * t27375;
    let t105909 = t1468 * t4343;
    let t105919 = t605 * t5962;
    let t105923 = t6075 * t775;
    let t105924 = t25207 * t105923;
    let t105928 = t98631 * t580 * t1583;
    (t105902, t105906, t105909, t105919, t105923, t105924, t105928)
}
