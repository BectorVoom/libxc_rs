//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 703/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk703(t13296: f64, t569: f64, t568: f64, t12969: f64, t13397: f64, t912: f64, t587: f64, t6915: f64, t6914: f64, t13402: f64, t2488: f64, t2487: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13457 = t569 * t13296;
    let t13458 = t568 * t13457;
    let t13463 = 0.17875244975925213335e0_f64 * t12969;
    let t13465 = t912 * t13397;
    let t13466 = t587 * t13465;
    let t13468 = t6915 * t13397;
    let t13469 = t6914 * t13468;
    let t13471 = t2488 * t13402;
    let t13472 = t2487 * t13471;
    (t13457, t13458, t13463, t13465, t13466, t13468, t13469, t13471, t13472)
}
