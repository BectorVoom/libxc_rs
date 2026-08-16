//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 629/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk629(t5559: f64, t5560: f64, t1464: f64, t242: f64, t366: f64, t5: f64) -> (f64, f64, f64) {
    let t5562 = 0.15154381759259259259e-2_f64 * t5559 * t5560;
    let t5588 = 0.50257692321302641125e0_f64 * t1464 * t242;
    let t5589 = t5 * t366;
    (t5562, t5588, t5589)
}
