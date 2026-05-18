//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 706/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk706<F: Float>(t6298: F, t3433: F, t2425: F, t568: F, t2467: F, t514: F, t211: F, t2472: F, t185: F, t3551: F, t3554: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t6299 = F::new(4.0) / F::new(45.0) * t6298;
    let t6300 = F::new(4.0) / F::new(135.0) * t3433;
    let t6301 = t2425 * t568;
    let t6302 = F::new(4.0) / F::new(45.0) * t6301;
    let t6303 = t514 * t2467;
    let t6304 = t211 * t6303;
    let t6305 = F::new(8.0) / F::new(45.0) * t6304;
    let t6306 = t514 * t2472;
    let t6307 = t185 * t6306;
    let t6308 = F::new(8.0) / F::new(45.0) * t6307;
    let t6309 = F::new(8.0) / F::new(135.0) * t3551;
    let t6310 = F::new(4.0) / F::new(135.0) * t3554;
    (t6299, t6300, t6301, t6302, t6303, t6304, t6305, t6306, t6307, t6308, t6309, t6310)
}
