//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 707/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk707(t420: f64, t9653: f64, t701: f64, t2440: f64, t9577: f64, t9571: f64, t3806: f64, t9583: f64, t2347: f64, t703: f64, t2320: f64, t9592: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9654 = t420 * t9653;
    let t9655 = t701 * t9654;
    let t9657 = t2440 * t9577;
    let t9658 = t9657 * t9571;
    let t9659 = t420 * t9658;
    let t9660 = t701 * t9659;
    let t9662 = t3806 * t9583;
    let t9663 = t701 * t9662;
    let t9665 = t703 * t2347;
    let t9666 = t9665 * t9571;
    let t9667 = t420 * t9666;
    let t9668 = t701 * t9667;
    let t9670 = t2320 * t9592;
    (t9655, t9658, t9660, t9663, t9666, t9668, t9670)
}
