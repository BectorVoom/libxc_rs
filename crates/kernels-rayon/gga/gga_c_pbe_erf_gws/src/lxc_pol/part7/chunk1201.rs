//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1201/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1201(t2168: f64, t6269: f64, t6523: f64, t6524: f64, t3138: f64, t3139: f64, t6177: f64, t6360: f64, t1: f64, t16192: f64, t191: f64, t745: f64, t816: f64) -> (f64, f64, f64, f64) {
    let t21355 = 3.0_f64 / 8.0_f64 * t2168 * t6523 * t6269 * t6524;
    let t21359 = 3.0_f64 / 8.0_f64 * t3138 * t3139 * t6177 * t6360;
    let t21361 = t191 * t16192 * t1;
    let t21366 = t816 * t745;
    (t21355, t21359, t21361, t21366)
}
