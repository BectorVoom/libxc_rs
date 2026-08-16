//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1154/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1154(t2001: f64, t5816: f64, t1988: f64, t9577: f64, t1095: f64, t1980: f64, t30058: f64, t5655: f64, t1967: f64, t9531: f64, t1901: f64, t7614: f64) -> (f64, f64, f64, f64, f64) {
    let t39937 = t2001 * t5816;
    let t39939 = t1988 * t9577;
    let t39944 = t1980 * t30058 * t1095 * t5655;
    let t39946 = t1967 * t9531;
    let t39948 = t7614 * t1901;
    (t39937, t39939, t39944, t39946, t39948)
}
