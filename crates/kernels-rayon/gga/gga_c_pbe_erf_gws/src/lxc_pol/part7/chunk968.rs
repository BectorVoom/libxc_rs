//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 968/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk968(t401: f64, t5049: f64, t1251: f64, t1718: f64, t5034: f64, t16964: f64, t16966: f64, t16968: f64, t16976: f64, t16981: f64, t16987: f64, t16989: f64, t16997: f64, t17018: f64, t25: f64, t657: f64) -> f64 {
    let t17919 = t401 * t5049;
    let t17927 = t1251 * t1718;
    let t17929 = t401 * t5034;
    let t17931 = -0.63985185185185185184e-1_f64 * t16964 + 0.47988888888888888888e-1_f64 * t16966 + 0.53320987654320987654e-1_f64 * t16968 - 0.10664197530864197531e0_f64 * t16976 - 0.35991666666666666667e-1_f64 * t16981 + 0.21595e0_f64 * t16989 - 0.86380000000000000002e0_f64 * t16997 + 0.17777777777777777778e-1_f64 * t17919 + 0.16e0_f64 * t25 * t657 * t17018 + 0.39999999999999999999e-1_f64 * t25 * t657 * t16987 + 0.88888888888888888889e-1_f64 * t17927 + 0.10666666666666666667e0_f64 * t17929;
    t17931
}
