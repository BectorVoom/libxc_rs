//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 777/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk777(t12635: f64, t587: f64, t12355: f64, t643: f64, t642: f64, t639: f64, t12350: f64, t5401: f64, t5400: f64, t2601: f64, t3553: f64, t1621: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12637 = 4.0_f64 / 15.0_f64 * t587 * t12635;
    let t12638 = t643 * t12355;
    let t12639 = t642 * t12638;
    let t12641 = 4.0_f64 / 45.0_f64 * t639 * t12639;
    let t12642 = t5401 * t12350;
    let t12643 = t5400 * t12642;
    let t12645 = 32.0_f64 / 81.0_f64 * t639 * t12643;
    let t12646 = t2601 * t3553;
    let t12647 = t1621 * t12646;
    (t12637, t12638, t12639, t12641, t12642, t12643, t12645, t12646, t12647)
}
