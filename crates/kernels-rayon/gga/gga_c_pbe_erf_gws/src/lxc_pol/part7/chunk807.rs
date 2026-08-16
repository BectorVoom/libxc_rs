//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 807/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk807(t6277: f64, t858: f64, t2407: f64, t6672: f64, t2118: f64, t6670: f64, t822: f64) -> (f64, f64, f64, f64, f64) {
    let t6673 = t858 * t6277;
    let t6674 = t2407 * t6673;
    let t6676 = t6672 * t6674 / 8.0_f64;
    let t6677 = t2118 * t6670;
    let t6678 = t822 * t6677;
    (t6673, t6674, t6676, t6677, t6678)
}
