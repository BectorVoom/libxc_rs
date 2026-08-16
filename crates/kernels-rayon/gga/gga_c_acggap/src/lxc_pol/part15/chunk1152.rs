//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1152/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1152(t1488: f64, t2030: f64, t2313: f64, t2001: f64, t5551: f64, t1856: f64, t7605: f64, t5811: f64, t5816: f64, t1988: f64, t9577: f64, t1095: f64, t1980: f64, t30058: f64, t5655: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t39928 = t2030 * t1488 * t2313;
    let t39930 = t2001 * t5551;
    let t39932 = t7605 * t1856;
    let t39934 = t2001 * t5811;
    let t39937 = t2001 * t5816;
    let t39939 = t1988 * t9577;
    let t39944 = t1980 * t30058 * t1095 * t5655;
    (t39928, t39930, t39932, t39934, t39937, t39939, t39944)
}
