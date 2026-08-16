//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 711/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk711(t676: f64, t9692: f64, t27: f64, t89: f64, t10: f64, t242: f64, t3050: f64, t1636: f64, t714: f64, t669: f64, t8608: f64, t666: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9693 = t676 * t9692;
    let t9695 = t89 * t27 * t9693;
    let t9698 = t10 * t3050 * t242;
    let t9699 = 14.0_f64 / 81.0_f64 * t9698;
    let t9701 = t89 * t1636 * t714;
    let t9703 = t669 * t8608;
    let t9705 = t89 * t666 * t9703;
    (t9693, t9695, t9698, t9699, t9701, t9703, t9705)
}
