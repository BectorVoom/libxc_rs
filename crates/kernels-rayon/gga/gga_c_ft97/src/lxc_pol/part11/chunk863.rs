//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 863/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk863(t1527: f64, t37315: f64, t419: f64, t37264: f64, t3088: f64, t37320: f64, t1725: f64, t8093: f64, t7705: f64, t7789: f64, t8106: f64, t11262: f64, t7807: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t37696 = t419 * t1527 * t37315;
    let t37699 = t419 * t1527 * t37264;
    let t37702 = t419 * t3088 * t37320;
    let t37704 = t1725 * t8093;
    let t37707 = t419 * t7705 * t7789;
    let t37709 = t1725 * t8106;
    let t37712 = t419 * t11262 * t7807;
    (t37696, t37699, t37702, t37704, t37707, t37709, t37712)
}
