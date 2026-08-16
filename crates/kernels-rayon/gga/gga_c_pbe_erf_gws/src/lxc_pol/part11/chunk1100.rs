//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1100/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1100(t173: f64, t184: f64, t199: f64, t47598: f64, t47611: f64, t30856: f64, t40474: f64, t3429: f64, t3454: f64, t5548: f64, t587: f64, t1815: f64, t40396: f64, t639: f64, t954: f64) -> (f64, f64, f64, f64, f64) {
    let t47616 = 2.0_f64 / 15.0_f64 * t173 * (t47598 + t47611) * t184 * t199;
    let t47617 = 16.0_f64 / 81.0_f64 * t30856;
    let t47618 = 64.0_f64 / 27.0_f64 * t40474;
    let t47622 = 16.0_f64 / 15.0_f64 * t587 * t5548 * t3429 * t3454;
    let t47626 = 16.0_f64 / 45.0_f64 * t639 * t1815 * t40396 * t954;
    (t47616, t47617, t47618, t47622, t47626)
}
