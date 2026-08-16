//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 552/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk552(t2679: f64, t948: f64, t9796: f64, t7809: f64, t822: f64) -> (f64, f64) {
    let t9797 = t948 * t2679;
    let t9798 = t9796 * t9797;
    let t9799 = 0.76685851907841499352e0_f64 * t9798;
    let t9800 = t822 * t7809;
    (t9799, t9800)
}
