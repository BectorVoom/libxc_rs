//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 234/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk234(t265: f64, t735: f64, t153: f64, t274: f64, t542: f64, t168: f64, t703: f64) -> (f64, f64, f64) {
    let t737 = 2.0_f64 / 45.0_f64 * t265 * t735;
    let t744 = 0.56945186695483624892e0_f64 * t153 * t542 * t274;
    let t751 = t168 * t703;
    (t737, t744, t751)
}
