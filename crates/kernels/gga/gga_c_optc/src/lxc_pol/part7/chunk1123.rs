//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1123/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1123<F: Float>(t23620: F, t23622: F, t23624: F, t23626: F, t23630: F, t23633: F, t23635: F, t23637: F, t23640: F, t23644: F, t23647: F, t23642: F, t25: F, t794: F) -> (F, F) {
    let t23649 = -F::cast_from(0.51702222222222222221e1_f64) * t23620 - F::cast_from(0.34468148148148148146e1_f64) * t23622 + F::cast_from(0.25851111111111111111e1_f64) * t23624 + F::cast_from(0.28723456790123456789e1_f64) * t23626 - F::cast_from(0.57446913580246913579e1_f64) * t23630 - F::cast_from(0.19388333333333333333e1_f64) * t23633 + F::cast_from(0.8042567901234567901e1_f64) * t23635 - F::cast_from(0.10340444444444444444e2_f64) * t23637 + F::cast_from(0.2585111111111111111e2_f64) * t23640 + F::cast_from(0.11633e2_f64) * t23644 + F::cast_from(0.29556e-1_f64) * t23647;
    let t23651 = t25 * t794 * t23642;
    (t23649, t23651)
}
