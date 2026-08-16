//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1067/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1067(t19234: f64, t5761: f64, t127: f64, t1504: f64, t1533: f64, t19216: f64, t19219: f64, t19229: f64, t19232: f64, t19236: f64, t19240: f64, t19242: f64, t19249: f64, t19254: f64, t5645: f64, t5825: f64, t5837: f64) -> f64 {
    let t19256 = t5761 * t19234;
    let t19258 = -t19216 + t19219 - 0.1762848e3_f64 * t127 * t5825 * t1504 * t1533 + 0.2350464e2_f64 * t127 * t5837 * t5645 + t19229 - t19232 - t19236 - t19240 + 4.0_f64 * t19242 - t19249 + 0.1175232e2_f64 * t19254 + 0.783488e1_f64 * t19256;
    t19258
}
