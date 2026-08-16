//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 916/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk916(t2624: f64, t2679: f64, t9800: f64, t5638: f64, t6574: f64, t822: f64) -> (f64, f64, f64, f64) {
    let t9801 = t2624 * t2679;
    let t9803 = 0.19171462976960374838e1_f64 * t9800 * t9801;
    let t9804 = t5638 * t6574;
    let t9805 = t822 * t9804;
    (t9801, t9803, t9804, t9805)
}
