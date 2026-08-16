//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1202/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1202(t21028: f64, t858: f64, t866: f64, t867: f64, t2168: f64, t2170: f64, t6177: f64, t6220: f64, t2121: f64, t337: f64, t5: f64, t6084: f64) -> (f64, f64, f64) {
    let t21378 = t866 * t867 * t858 * t21028 / 96.0_f64;
    let t21382 = t2168 * t2170 * t6177 * t6220 / 8.0_f64;
    let t21385 = t2121 * t337 * t5 * t6084;
    (t21378, t21382, t21385)
}
