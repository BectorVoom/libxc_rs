//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1191/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1191(t10295: f64, t17288: f64, t17277: f64, t3366: f64, t2358: f64, t27232: f64, t10629: f64, t5227: f64, t10632: f64, t161: f64, t1841: f64, t24884: f64, t2576: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t32093 = 12.0_f64 * t17288 * t10295;
    let t32095 = 2.0_f64 * t17277 * t3366;
    let t32099 = 4.0_f64 * t27232 * t2358;
    let t32104 = 0.34180116578409885704e-2_f64 * t5227 * t10629;
    let t32106 = 0.51270174867614828558e-2_f64 * t5227 * t10632;
    let t32110 = 0.51270174867614828558e-2_f64 * t1841 * t24884 * t161 * t2576;
    (t32093, t32095, t32099, t32104, t32106, t32110)
}
