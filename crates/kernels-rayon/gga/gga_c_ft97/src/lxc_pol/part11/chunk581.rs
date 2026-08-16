//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 581/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk581(t1527: f64, t7789: f64, t419: f64, t1725: f64, t1744: f64, t173: f64, t1743: f64, t1736: f64, t7800: f64, t7765: f64, t420: f64, t3088: f64, t7807: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8093 = t1527 * t7789;
    let t8094 = t419 * t8093;
    let t8096 = t1725 * t1744;
    let t8098 = t173 * t1743;
    let t8099 = t419 * t8098;
    let t8101 = t1736 * t7800;
    let t8102 = t8101 * t7765;
    let t8103 = t420 * t8102;
    let t8104 = t419 * t8103;
    let t8106 = t3088 * t7807;
    (t8093, t8094, t8096, t8098, t8099, t8102, t8103, t8104, t8106)
}
