//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 853/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk853(t20: f64, t2004: f64, t5450: f64, t5942: f64, t5953: f64, t156: f64, t5926: f64, t670: f64, t1999: f64, t542: f64, t1673: f64, t1775: f64) -> (f64, f64, f64, f64, f64) {
    let t16492 = t5450 * t20 * t2004;
    let t16494 = t5953 * t5942;
    let t16498 = 0.43284165449459373508e0_f64 * t670 * t156 * t5926;
    let t16501 = 0.38474813732852776452e0_f64 * t670 * t542 * t1999;
    let t16502 = t1775 * t1673;
    (t16492, t16494, t16498, t16501, t16502)
}
