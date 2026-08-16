//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 939/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk939(t1986: f64, t1989: f64, t1879: f64, t5556: f64, t1827: f64, t418: f64, t5273: f64, t572: f64, t587: f64, t2735: f64, t611: f64, t185: f64) -> (f64, f64, f64, f64) {
    let t17490 = t1989 * t1986;
    let t17492 = t1879 * t5556;
    let t17493 = 32.0_f64 / 45.0_f64 * t17492;
    let t17498 = 16.0_f64 / 45.0_f64 * t587 * t1827 * t5273 * t572 * t418;
    let t17499 = t2735 * t611;
    let t17500 = t185 * t17499;
    (t17490, t17493, t17498, t17500)
}
