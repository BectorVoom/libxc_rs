//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 1041/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk1041(t1: f64, t632: f64, t27144: f64, t5972: f64, t3074: f64, t5700: f64, t5964: f64, t1038: f64, t1908: f64, t1954: f64, t5059: f64, t8884: f64) -> (f64, f64, f64, f64, f64) {
    let t27307 = t632 * t1;
    let t27309 = t27144 * t5972;
    let t27354 = t5964 * t3074 * t5700;
    let t27408 = t1038 * t1908 * t1954;
    let t27420 = t8884 * t5059;
    (t27307, t27309, t27354, t27408, t27420)
}
