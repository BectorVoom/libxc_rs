//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1074/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1074(t506: f64, t5832: f64, t2: f64, t39: f64, t481: f64, t10: f64, t1533: f64, t1564: f64, t19307: f64, t19312: f64, t19316: f64, t19318: f64, t19320: f64, t19322: f64, t19324: f64, t19338: f64, t19340: f64, t496: f64, t5780: f64) -> (f64, f64) {
    let t19342 = t5832 * t506;
    let t19344 = t2 * t39 * t481;
    let t19345 = t19342 * t19344;
    let t19347 = -t496 * t19307 / 2.0_f64 + t19312 + t19316 + 0.587616e2_f64 * t19318 - 0.293808e1_f64 * t19320 + 0.293808e1_f64 * t19322 + 0.91406933333333333333e1_f64 * t19324 + 6.0_f64 * t496 * t10 * t5780 * t481 - 36.0_f64 * t496 * t10 * t1564 * t1533 - t19338 - 0.3525696e2_f64 * t19340 - 0.391744e1_f64 * t19345;
    (t19344, t19347)
}
