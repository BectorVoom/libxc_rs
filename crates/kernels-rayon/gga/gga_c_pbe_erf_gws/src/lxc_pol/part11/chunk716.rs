//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 716/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk716(t3426: f64, t395: f64, t3430: f64, t3584: f64, t723: f64, t3398: f64, t586: f64) -> (f64, f64, f64, f64) {
    let t10825 = t395 * t3426;
    let t10827 = t395 * t3430;
    let t10841 = t3584 * t723;
    let t10843 = t3398 * t586;
    (t10825, t10827, t10841, t10843)
}
