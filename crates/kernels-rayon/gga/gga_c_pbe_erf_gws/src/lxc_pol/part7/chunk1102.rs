//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1102/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1102(t2112: f64, t745: f64, t2397: f64, t4453: f64, t2379: f64, t6745: f64, t2367: f64, t6151: f64, t2387: f64, t6744: f64, t833: f64, t2306: f64, t4422: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19714 = t2112 * t745;
    let t19722 = t4453 * t2397;
    let t19726 = t6745 * t2379;
    let t19728 = t2367 * t6151;
    let t19731 = t2387 * t6744 * t833;
    let t19733 = t2306 * t4422;
    (t19714, t19722, t19726, t19728, t19731, t19733)
}
