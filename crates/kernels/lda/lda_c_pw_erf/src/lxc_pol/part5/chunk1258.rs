//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1258/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1258<F: Float>(t2480: F, t5211: F, t2076: F, t6867: F, t18308: F, t18311: F, t18314: F, t18317: F, t18390: F, t185: F, t514: F, t7521: F) -> (F, F, F, F, F, F, F, F) {
    let t22584 = F::new(4.0) / F::new(5.0) * t5211 * t2480;
    let t22586 = F::new(4.0) / F::new(5.0) * t2076 * t6867;
    let t22587 = F::new(16.0) / F::new(9.0) * t18308;
    let t22588 = F::new(8.0) / F::new(45.0) * t18311;
    let t22589 = F::new(16.0) / F::new(45.0) * t18314;
    let t22590 = F::new(8.0) / F::new(27.0) * t18317;
    let t22591 = F::new(4.0) / F::new(15.0) * t18390;
    let t22593 = t185 * t514 * t7521;
    (t22584, t22586, t22587, t22588, t22589, t22590, t22591, t22593)
}
