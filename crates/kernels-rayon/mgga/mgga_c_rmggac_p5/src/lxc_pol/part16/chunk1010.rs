//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1010/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1010(t26857: f64, t8410: f64, t6355: f64, t8542: f64, t2283: f64, t38355: f64, t8571: f64, t8582: f64, t40278: f64, t8443: f64, t1704: f64, t352: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t47108 = t26857 * t8410;
    let t47110 = t6355 * t8542;
    let t47112 = t38355 * t2283;
    let t47114 = t8571 * t8582;
    let t47119 = t40278 * t8443;
    let t47124 = t1704 * t352;
    (t47108, t47110, t47112, t47114, t47119, t47124)
}
