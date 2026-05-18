//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1323/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1323<F: Float>(t12810: F, t12815: F, t12839: F, t12863: F, t12870: F, t21431: F, t21432: F, t21436: F, t21438: F, t21442: F, t21445: F, t21447: F, t21449: F) -> F {
    let t23247 = -t21431 - t21432 + t21436 - t21438 + t21442 + t21445 + t21447 - t21449 - t12810 + t12815 + t12839 + t12863 - t12870;
    t23247
}
