//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 556/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk556(t9823: f64, t9824: f64, t1966: f64, t1991: f64, t2028: f64, t9789: f64, t9790: f64, t9793: f64, t9799: f64, t9803: f64, t9809: f64, t9812: f64, t9815: f64, t9817: f64, t9822: f64) -> (f64, f64) {
    let t9826 = 0.29792074959875355558e-1_f64 * t9823 * t9824;
    let t9827 = t9789 - 0.25561950635947166451e1_f64 * t1966 * t9790 + 0.51123901271894332902e0_f64 * t1991 * t9793 - t9799 + t9803 - t9809 + t9812 + t9815 - 0.39722766613167140743e-1_f64 * t9817 * t2028 - t9822 + t9826;
    (t9826, t9827)
}
