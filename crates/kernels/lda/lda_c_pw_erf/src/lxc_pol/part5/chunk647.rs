//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 647/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk647<F: Float>(t2104: F, t2137: F, t1284: F, t1298: F, t2127: F, t2134: F, t511: F, t2114: F, t1958: F, t202: F, t184: F, t172: F, t1980: F) -> (F, F, F, F, F, F, F, F) {
    let t5190 = F::new(16.0) / F::new(45.0) * t2104 * t2137;
    let t5192 = F::new(16.0) / F::new(45.0) * t1284 * t2137;
    let t5194 = F::new(16.0) / F::new(45.0) * t1298 * t2127;
    let t5198 = F::new(8.0) / F::new(45.0) * t511 * t2134;
    let t5200 = F::new(16.0) / F::new(45.0) * t2114 * t2127;
    let t5210 = t202 * t1958;
    let t5211 = t5210 * t184;
    let t5214 = t172 * t1980;
    (t5190, t5192, t5194, t5198, t5200, t5210, t5211, t5214)
}
