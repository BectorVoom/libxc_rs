//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1252/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1252(t13544: f64, t20607: f64, t2277: f64, t3257: f64, t36814: f64, t3836: f64, t45017: f64, t45620: f64, t45703: f64, t49087: f64, t49845: f64, t49852: f64, t49857: f64, t49859: f64, t49861: f64, t49875: f64, t6384: f64, t904: f64, t929: f64) -> f64 {
    let t49879 = -t49845 - 7.0_f64 / 144.0_f64 * t45620 + t49852 - t49857 - t49859 - t49861 + 11.0_f64 / 768.0_f64 * t2277 * t3257 * t36814 * t45017 - 15.0_f64 / 64.0_f64 * t929 * t6384 * t904 * t49087 + 7.0_f64 / 96.0_f64 * t45703 + t49875 - 3.0_f64 / 16.0_f64 * t20607 * t3836 * t13544;
    t49879
}
