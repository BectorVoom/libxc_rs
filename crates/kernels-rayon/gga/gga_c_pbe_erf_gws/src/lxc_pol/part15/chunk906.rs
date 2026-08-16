//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 906/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk906(t247: f64, t7908: f64, t251: f64, t2626: f64, t5018: f64, t1820: f64, t1898: f64, t2615: f64, t1648: f64, t2643: f64, t1733: f64, t2596: f64) -> (f64, f64, f64, f64, f64) {
    let t7909 = t7908 * t247;
    let t7910 = t7909 * t251;
    let t7913 = t5018 * t2626;
    let t7915 = 16.0_f64 / 45.0_f64 * t1820 * t7913;
    let t7917 = 8.0_f64 / 45.0_f64 * t2615 * t1898;
    let t7919 = 16.0_f64 / 135.0_f64 * t1648 * t2643;
    let t7920 = t2596 * t1733;
    (t7910, t7915, t7917, t7919, t7920)
}
