//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 759/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk759(t11187: f64, t11250: f64, t11264: f64, t5895: f64, t5898: f64, t5977: f64, t5986: f64, t5988: f64, t5993: f64, t8387: f64, t8390: f64, t8467: f64) -> f64 {
    let t12433 = -0.59261670986728442646e-2_f64 * t11264 + 0.26942026523072870461e-1_f64 * t11187 + 0.94516221669423353502e-1_f64 * t11250 + 0.19753890328909480882e-1_f64 * t8467 - 0.11852334197345688529e-1_f64 * t8387 - 0.14862827083471493416e-2_f64 * t8390 - t5895 - t5898 - t5977 - t5986 + t5988 - t5993;
    t12433
}
