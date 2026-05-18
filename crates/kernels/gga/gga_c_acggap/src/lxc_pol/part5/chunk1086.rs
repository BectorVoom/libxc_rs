//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1086/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1086<F: Float>(t14892: F, t192: F, t5506: F, t14898: F, t14900: F, t14902: F, t14904: F, t11683: F, t11696: F, t234: F, t34: F, t821: F) -> (F, F, F, F, F, F, F, F, F) {
    let t19441 = F::new(0.65061487801810439052e-1) * t14892;
    let t19444 = t192 * t5506;
    let t19451 = F::new(0.36622894612013090108e-3) * t14898;
    let t19452 = F::new(0.97661052298701573622e-3) * t14900;
    let t19453 = F::new(0.2077903092681775651e3) * t14902;
    let t19454 = F::new(0.46785788981077169656e1) * t14904;
    let t19455 = F::new(0.70178683471615754484e1) * t11683;
    let t19456 = F::new(12.0) * t11696;
    let t19461 = t234 * t34 * t821;
    (t19441, t19444, t19451, t19452, t19453, t19454, t19455, t19456, t19461)
}
