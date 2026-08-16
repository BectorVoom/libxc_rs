//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 701/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk701(t1457: f64, t285: f64, t545: f64, t1368: f64, t762: f64, t147: f64, t366: f64) -> (f64, f64, f64) {
    let t5690 = t1457 * t545 * t285;
    let t5694 = 0.87170224553660758101e-3_f64 * t762 * t1368 * t285;
    let t5697 = t366 * t147;
    (t5690, t5694, t5697)
}
