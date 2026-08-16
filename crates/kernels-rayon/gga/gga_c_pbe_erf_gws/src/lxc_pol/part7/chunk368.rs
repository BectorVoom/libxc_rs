//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 368/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk368(t1464: f64, t159: f64, t285: f64, t169: f64, t274: f64, t301: f64, t366: f64, t5: f64, t784: f64) -> (f64, f64, f64) {
    let t1467 = 0.13559812708347229038e-2_f64 * t1464 * t159 * t285;
    let t1471 = 0.19816831758676854261e0_f64 * t169 * t366 * t274 * t301;
    let t1472 = t5 * t784;
    (t1467, t1471, t1472)
}
