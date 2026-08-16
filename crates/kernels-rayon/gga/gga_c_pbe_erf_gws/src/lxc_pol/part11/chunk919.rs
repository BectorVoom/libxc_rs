//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 919/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk919(t18753: f64, t18801: f64, t18839: f64, t18910: f64, t40: f64, t60: f64, t18639: f64, t18865: f64, t470: f64, t4737: f64, t39: f64, t55: f64, t59: f64, t87: f64) -> (f64, f64, f64) {
    let t18914 = t40 * t60 * (t18753 + t18801 + t18839 + t18910);
    let t18920 = 0.12304676425209353917e5_f64 * t470 * t18865 * t18639 * t4737;
    let t18924 = 24.0_f64 * t39 * t55 * t59 * t87;
    (t18914, t18920, t18924)
}
