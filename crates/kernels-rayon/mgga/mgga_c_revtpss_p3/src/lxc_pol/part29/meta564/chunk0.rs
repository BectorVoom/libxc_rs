//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1908/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1908(t25978: f64, t5629: f64, t1885: f64, t94459: f64, t26024: f64, t5661: f64, t14054: f64, t25986: f64, t2661: f64, t13874: f64, t7271: f64, t14046: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t98222 = t25978 * t5629;
    let t98224 = t94459 * t1885;
    let t98226 = t26024 * t5661;
    let t98229 = t2661 * t25986 * t14054;
    let t98231 = t7271 * t13874;
    let t98235 = t2661 * t25986 * t14046;
    (t98222, t98224, t98226, t98229, t98231, t98235)
}
