//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 478/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk478(t325: f64, t883: f64, t900: f64, t6574: f64, t823: f64, t1984: f64) -> (f64, f64, f64, f64) {
    let t7784 = t883 * t325;
    let t7785 = t900 * t7784;
    let t7802 = t823 * t6574;
    let t7803 = t1984 * t7802;
    (t7784, t7785, t7802, t7803)
}
