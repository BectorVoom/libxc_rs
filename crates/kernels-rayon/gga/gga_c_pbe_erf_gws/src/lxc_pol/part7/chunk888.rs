//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 888/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk888(t16883: f64, t1663: f64, t1820: f64, t1821: f64, t4352: f64, t562: f64, t1680: f64, t1740: f64, t5516: f64, t612: f64, t5125: f64, t5133: f64) -> (f64, f64, f64, f64, f64) {
    let t16884 = 128.0_f64 / 45.0_f64 * t16883;
    let t16889 = 64.0_f64 / 15.0_f64 * t1820 * t1821 * t562 * t1663 * t4352;
    let t16890 = t1680 * t1740;
    let t16891 = 32.0_f64 / 15.0_f64 * t16890;
    let t16893 = 16.0_f64 / 5.0_f64 * t5516 * t612;
    let t16895 = t1820 * t5125 * t5133;
    (t16884, t16889, t16891, t16893, t16895)
}
