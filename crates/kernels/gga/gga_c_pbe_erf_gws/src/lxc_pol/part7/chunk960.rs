//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 960/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk960<F: Float>(t1557: F, t19292: F, t1: F, t1533: F, t392: F, t485: F, t497: F, t501: F, t5837: F, t506: F, t5832: F, t2: F, t39: F, t481: F, t10: F, t1564: F, t19307: F, t19312: F, t19316: F, t19318: F, t19320: F, t19322: F, t496: F, t5780: F) -> (F, F, F) {
    let t19324 = t1557 * t19292;
    let t19336 = t1533 * t1 * t392;
    let t19337 = t485 * t497 * t19336;
    let t19338 = 0.116921e2 * t19337;
    let t19340 = t501 * t5837 * t19336;
    let t19342 = t5832 * t506;
    let t19344 = t2 * t39 * t481;
    let t19345 = t19342 * t19344;
    let t19347 = -t496 * t19307 / 2.0 + t19312 + t19316 + 0.587616e2 * t19318 - 0.293808e1 * t19320 + 0.293808e1 * t19322 + 0.91406933333333333333e1 * t19324 + 6.0 * t496 * t10 * t5780 * t481 - 36.0 * t496 * t10 * t1564 * t1533 - t19338 - 0.3525696e2 * t19340 - 0.391744e1 * t19345;
    (t19338, t19344, t19347)
}
