//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1142/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1142(t47450: f64, t587: f64, t7435: f64, t12460: f64, t1820: f64, t995: f64, t12805: f64, t2615: f64, t1017: f64, t17260: f64, t1022: f64, t12513: f64, t1620: f64, t1809: f64) -> (f64, f64, f64, f64, f64) {
    let t48169 = 64.0_f64 / 27.0_f64 * t587 * t7435 * t47450;
    let t48173 = 256.0_f64 / 81.0_f64 * t1820 * t7435 * t12460 * t995;
    let t48175 = 16.0_f64 / 9.0_f64 * t2615 * t12805;
    let t48179 = 128.0_f64 / 81.0_f64 * t587 * t17260 * t12460 * t1017;
    let t48183 = 32.0_f64 / 45.0_f64 * t1620 * t1809 * t12513 * t1022;
    (t48169, t48173, t48175, t48179, t48183)
}
