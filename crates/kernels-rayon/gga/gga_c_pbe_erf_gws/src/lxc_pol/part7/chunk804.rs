//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 804/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk804(t2302: f64, t2323: f64, t56: f64, t931: f64, t19: f64, t6385: f64, t858: f64, t884: f64, t4394: f64, t820: f64, t274: f64, t6161: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6656 = t2323 * t2302;
    let t6658 = t56 * t931;
    let t6659 = t6658 * t19;
    let t6661 = t6659 * t858 * t6385;
    let t6663 = t884 * t6661 / 4.0_f64;
    let t6664 = t820 * t4394;
    let t6665 = t274 * t6161;
    (t6656, t6658, t6659, t6661, t6663, t6664, t6665)
}
