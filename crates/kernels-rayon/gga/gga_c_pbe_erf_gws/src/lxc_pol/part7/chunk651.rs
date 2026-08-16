//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 651/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk651(t1823: f64, t5125: f64, t1820: f64, t1651: f64, t597: f64, t1828: f64, t587: f64, t1769: f64, t562: f64, t1821: f64, t1630: f64, t649: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5126 = t5125 * t1823;
    let t5127 = t1820 * t5126;
    let t5128 = 32.0_f64 / 45.0_f64 * t5127;
    let t5129 = t1651 * t597;
    let t5130 = t5129 * t1828;
    let t5131 = t587 * t5130;
    let t5132 = 16.0_f64 / 45.0_f64 * t5131;
    let t5133 = t1769 * t562;
    let t5134 = t1821 * t5133;
    let t5136 = 8.0_f64 / 15.0_f64 * t1820 * t5134;
    let t5137 = t1630 * t649;
    (t5126, t5128, t5129, t5130, t5132, t5133, t5134, t5136, t5137)
}
