//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1182/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1182<F: Float>(t17461: F, t2505: F, t3455: F, t12974: F, t12978: F, t1971: F, t3965: F, t3968: F, t4479: F, t10169: F, t10172: F, t17444: F, t17445: F, t17446: F, t17448: F, t17450: F, t17452: F, t17454: F, t17457: F, t17459: F) -> (F, F, F, F, F, F) {
    let t17462 = 8.0 / 135.0 * t17461;
    let t17464 = 4.0 / 15.0 * t3455 * t2505;
    let t17465 = 32.0 / 135.0 * t12974;
    let t17466 = 128.0 / 243.0 * t12978;
    let t17470 = 64.0 / 45.0 * t3965 * t4479 * t1971 * t3968;
    let t17471 = t17444 + t17445 + t17446 - t17448 + t17450 + t17452 + t17454 + 8.0 / 3.0 * t10169 - t10172 - t17457 - t17459 - t17462 + t17464 - t17465 + t17466 + t17470;
    (t17462, t17464, t17465, t17466, t17470, t17471)
}
