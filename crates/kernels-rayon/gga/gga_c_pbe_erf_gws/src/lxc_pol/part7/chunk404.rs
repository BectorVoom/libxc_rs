//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 404/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk404(t1672: f64, t198: f64, t185: f64, t579: f64, t583: f64, t562: f64, t181: f64, t184: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1673 = t1672 * t198;
    let t1675 = 4.0_f64 / 135.0_f64 * t185 * t1673;
    let t1676 = t579 * t583;
    let t1677 = 8.0_f64 / 45.0_f64 * t1676;
    let t1678 = t562 * t562;
    let t1679 = t1678 * t181;
    let t1680 = t1679 * t184;
    (t1673, t1675, t1677, t1678, t1679, t1680)
}
