//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 242/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk242(t265: f64, t735: f64, t256: f64, t267: f64, t566: f64, t581: f64, t585: f64, t595: f64, t614: f64, t621: f64, t635: f64, t638: f64, t647: f64, t665: f64, t708: f64, t716: f64, t722: f64, t725: f64, t732: f64) -> (f64, f64) {
    let t737 = 2.0_f64 / 45.0_f64 * t265 * t735;
    let t738 = t566 + t581 + t585 + t595 - t614 + t621 + t635 + t638 + t647 - t665 + t708 * t256 / 3.0_f64 + t716 + t722 + t725 - t732 * t267 / 15.0_f64 - t737;
    (t737, t738)
}
