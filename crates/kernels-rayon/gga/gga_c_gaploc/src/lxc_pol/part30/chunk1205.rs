//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1205/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1205(t10632: f64, t5227: f64, t161: f64, t1841: f64, t24884: f64, t2576: f64, t29160: f64, t29162: f64, t29184: f64, t29186: f64, t29210: f64, t29212: f64, t29224: f64, t29226: f64, t29230: f64, t29233: f64, t29242: f64, t32104: f64) -> f64 {
    let t32106 = 0.51270174867614828558e-2_f64 * t5227 * t10632;
    let t32110 = 0.51270174867614828558e-2_f64 * t1841 * t24884 * t161 * t2576;
    let t32111 = t29160 - t29162 + t29184 + t29186 - t29210 - t29212 - t29224 - t29226 - t29230 + t29233 - t29242 - t32104 + t32106 + t32110;
    t32111
}
