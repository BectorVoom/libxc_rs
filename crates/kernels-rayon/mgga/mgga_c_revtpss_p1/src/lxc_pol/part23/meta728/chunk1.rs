//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2497/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2497(t10199: f64, t1514: f64, t2289: f64, t4264: f64, t10227: f64, t97: f64, t10241: f64, t105: f64, t4288: f64, t4398: f64, t9372: f64, t1469: f64, t2608: f64, t4401: f64, t606: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t49698 = t10199 * t1514;
    let t49700 = t2289 * t4264;
    let t49701 = 22.0_f64 / 3.0_f64 * t49700;
    let t49777 = t97 * t10227;
    let t49787 = t105 * t10241;
    let t49817 = t2289 * t4288;
    let t49818 = 11.0_f64 / 3.0_f64 * t49817;
    let t49866 = t4398 * t9372;
    let t49876 = t4401 * t2608 * t1469 * t606;
    (t49698, t49701, t49777, t49787, t49818, t49866, t49876)
}
