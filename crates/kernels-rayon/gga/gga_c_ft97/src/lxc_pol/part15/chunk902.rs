//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 902/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk902(t37345: f64, t4418: f64, t89: f64, t1636: f64, t4437: f64, t46256: f64, t4432: f64, t7773: f64, t46320: f64, t4515: f64, t8282: f64, t1771: f64, t4531: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t57527 = t89 * t37345 * t4418;
    let t57620 = t89 * t1636 * t4437;
    let t57627 = 56.0_f64 / 81.0_f64 * t46256;
    let t57718 = t89 * t7773 * t4432;
    let t57767 = 56.0_f64 / 243.0_f64 * t46320;
    let t57980 = t8282 * t4515;
    let t58140 = t1771 * t4531;
    (t57527, t57620, t57627, t57718, t57767, t57980, t58140)
}
