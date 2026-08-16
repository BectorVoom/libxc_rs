//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1408/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1408(t5170: f64, t26195: f64, t26197: f64, t12366: f64, t17656: f64, t1056: f64, t8582: f64, t5186: f64, t2993: f64, t44583: f64, t5171: f64, t17436: f64, t34434: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t59157 = t5170 * t5170;
    let t59160 = 0.24954977986735470917e5_f64 * t26195 * t59157 * t26197;
    let t59162 = 24.0_f64 * t12366 * t17656;
    let t59165 = 24.0_f64 * t8582 * t59157 * t1056;
    let t59166 = t5186 * t5186;
    let t59169 = 6.0_f64 * t2993 * t59166 * t1056;
    let t59171 = 12.0_f64 * t44583 * t5171;
    let t59173 = 0.38596378373162651572e3_f64 * t34434 * t17436;
    (t59157, t59160, t59162, t59165, t59166, t59169, t59171, t59173)
}
