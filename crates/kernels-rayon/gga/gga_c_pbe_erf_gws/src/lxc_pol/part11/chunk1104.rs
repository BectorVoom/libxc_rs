//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1104/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1104(t31225: f64, t12647: f64, t2612: f64, t1017: f64, t1820: f64, t1885: f64, t40676: f64, t16845: f64, t18196: f64, t47675: f64, t47676: f64, t47677: f64, t47678: f64, t47679: f64, t47683: f64, t47684: f64) -> (f64, f64, f64, f64) {
    let t47685 = 32.0_f64 / 135.0_f64 * t31225;
    let t47687 = 16.0_f64 / 5.0_f64 * t2612 * t12647;
    let t47691 = 16.0_f64 / 15.0_f64 * t1820 * t1885 * t40676 * t1017;
    let t47692 = t47675 + t18196 + t47676 + t47677 - t47678 + t47679 - t47683 - t16845 - t47684 + t47685 + t47687 - t47691;
    (t47685, t47687, t47691, t47692)
}
