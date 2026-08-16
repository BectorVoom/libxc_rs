//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 930/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk930(t34975: f64, t34976: f64, t571: f64, t8455: f64, t1368: f64, t16503: f64, t3369: f64, t9163: f64, t2186: f64, t9731: f64, t2320: f64, t38370: f64) -> (f64, f64, f64, f64) {
    let t45499 = t34975 * t34976 * t571 * t8455;
    let t45503 = t16503 * t3369 * t1368 * t9163;
    let t45505 = t2186 * t9731;
    let t45507 = t38370 * t2320;
    (t45499, t45503, t45505, t45507)
}
