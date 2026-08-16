//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 890/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk890(t219: f64, t5463: f64, t1620: f64, t1811: f64, t16675: f64, t2559: f64, t587: f64, t1627: f64, t4898: f64, t1815: f64, t422: f64, t5097: f64, t626: f64, t639: f64) -> (f64, f64, f64, f64) {
    let t16904 = t5463 * t219;
    let t16906 = t1620 * t16904 * t1811;
    let t16907 = 64.0_f64 / 135.0_f64 * t16906;
    let t16910 = 16.0_f64 / 3.0_f64 * t587 * t2559 * t16675;
    let t16912 = 16.0_f64 / 15.0_f64 * t1627 * t4898;
    let t16917 = 16.0_f64 / 45.0_f64 * t639 * t1815 * t5097 * t626 * t422;
    (t16907, t16910, t16912, t16917)
}
