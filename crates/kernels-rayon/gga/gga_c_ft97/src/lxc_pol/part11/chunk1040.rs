//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1040/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1040(t41698: f64, t683: f64, t92: f64, t41490: f64, t3051: f64, t685: f64, t1771: f64, t2414: f64, t2406: f64, t41446: f64, t41448: f64, t9568: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t41700 = t92 * t683 * t41698;
    let t41703 = t92 * t683 * t41490;
    let t41705 = t3051 * t685;
    let t41707 = t1771 * t2414;
    let t41709 = t1771 * t2406;
    let t41711 = t41446 * t41448;
    let t41713 = t92 * t9568 * t41711;
    (t41700, t41703, t41705, t41707, t41709, t41711, t41713)
}
