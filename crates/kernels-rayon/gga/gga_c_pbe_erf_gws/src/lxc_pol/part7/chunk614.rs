//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 614/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk614(t43: f64, t1336: f64, t461: f64, t428: f64, t726: f64, t1402: f64, t418: f64, t1407: f64, t4352: f64, t4360: f64, t47: f64, t728: f64, t1412: f64, t422: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t44 = t43 <= zeta_threshold;
    let t4753 = t1336 * t461;
    let t4754 = 36.0_f64 * t4753;
    let t4755 = t1336 * t428;
    let t4756 = 36.0_f64 * t4755;
    let t4757 = 1.0_f64 / t726;
    let t4760 = t1402 * t418;
    let t4766 = piecewise3(t44, 0.0_f64, -8.0_f64 / 27.0_f64 * t4757 * t4352 + 4.0_f64 / 3.0_f64 * t4760 * t1407 + 4.0_f64 / 3.0_f64 * t47 * t4360);
    let t4767 = 1.0_f64 / t728;
    let t4770 = t1412 * t422;
    (t4753, t4754, t4755, t4756, t4757, t4760, t4766, t4767, t4770)
}
